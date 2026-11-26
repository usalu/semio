//! 🔎 Print readable Rust symbols from the original requester names identified by a sampled worker.
#![feature(rustc_private)]
extern crate rustc_demangle;
use std::io::{self, BufRead};
fn main() -> io::Result<()> {
    for line in io::stdin().lock().lines() {
        let line=line?;
        if let Some((ids, symbol)) = line.rsplit_once('\t') {
            println!("{ids}\t{:#}", rustc_demangle::demangle(symbol));
        }
    }
    Ok(())
}
