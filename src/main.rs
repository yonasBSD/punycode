use idna::punycode;
use clap::{Parser, Subcommand};
use log::{debug, error};

pub mod tests {
  pub mod default {
    use idna::punycode;

    pub fn list() {
      println!("Encode");
      println!("Decode");
    }

    pub fn run() {
      test_encode();
      test_decode();
    }

    pub fn test_encode() {
      print!("Running encode tests ... ");
      if let Some(result) = punycode::encode_str("マリウス") {
        assert_eq!(result, "gckvb8fzb");
      }
      println!("ok");
    }

    pub fn test_decode() {
      print!("Running decode tests ... ");
      if let Some(result) = punycode::decode_to_string("gckvb8fzb") {
        assert_eq!(result, "マリウス");
      }
      println!("ok");
    }
  }
}

#[derive(Parser)]
#[command(name = "punycode")]
#[command(author = "Yonas Yanfa <yonas@mail.lan>")]
#[command(version = "0.0.1")]
#[command(about = "Display idna::punycode", long_about = None)]
struct Cli {
    /// Enable debug mode
    #[arg(long)]
    debug: bool,

    /// Enable verbose mode
    #[arg(long)]
    verbose: bool,

    /// Encode
    #[arg(long)]
    encode: Option<String>,

    /// Decode
    #[arg(long)]
    decode: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run tests
    Test {
        /// Lists available tests
        #[arg(short, long)]
        list: bool,

        /// Run all tests
        #[arg(short, long)]
        all: bool,
    },
}

fn main() {
  let cli = Cli::parse();
  if cli.debug == true {
    std::env::set_var("RUST_LOG", "debug");
  }

  env_logger::init();

  if let Some(input) = cli.encode.as_deref() {
    debug!("{:#?}", input);
    if let Some(result) = punycode::encode_str(input) {
      println!("{}", result);
    } else {
      error!("Could not encode input.");
    }
  }
  else if let Some(input) = cli.decode.as_deref() {
    debug!("{:#?}", input);
    if let Some(result) = punycode::decode_to_string(input) {
      println!("{}", result);
    } else {
      error!("Could not encode input.");
    }
  }
  else {
    match &cli.command {
      Some(Commands::Test { list, all }) => {
        if *list {
          println!("Printing testing lists...");
          tests::default::list();
          std::process::exit(0)
        } else if *all {
          println!("Running all tests...");
          tests::default::run();
          std::process::exit(0)
        } else {
          println!("Running all tests...");
          tests::default::run();
          std::process::exit(0)
        }
      }
      None => {}
    }

    error!("Missing input.");
    std::process::exit(1)
  }
}
