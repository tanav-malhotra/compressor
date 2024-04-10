use flate2::write::GzEncoder;
use flate2::Compression;
use std::env;
use std::fs::File;
use std::io::{copy, BufReader};
use std::time::Instant;
use std::process;

fn main() {
    if env::args().len() != 3 {
        eprintln!("Usage: compressor <source> <target>");
        process::exit(0);        
    }

    let mut input = BufReader::new(File::open(env::args().nth(1).unwrap()).unwrap());
    let output = File::create(env::args().nth(2).unwrap()).unwrap();

    let mut encoder = GzEncoder::new(output, Compression::default());

    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    let elapsed_time = start.elapsed();
    println!(
        "Source size: {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("Target size: {:?}", output.metadata().unwrap().len());
    println!("Elapsed time: {:?}", elapsed_time);
}
