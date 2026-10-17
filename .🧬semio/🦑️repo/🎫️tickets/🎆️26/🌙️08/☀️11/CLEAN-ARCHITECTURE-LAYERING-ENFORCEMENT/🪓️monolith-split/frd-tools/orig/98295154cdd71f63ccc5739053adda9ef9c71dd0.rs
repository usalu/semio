//! 🥣️ Companion tests of print: chart mutation replay, real inference dispatch and the SQLite snapshot laws.
//!
//! 🧭️ The tests read the chart through its public API and read every fixture and schema document at run time from the print product directory, so no document is compiled into the test closure.
#![cfg(test)]
extern crate semio_framework_os_kernel as protocol;

use std::{collections::HashMap, path::{Path, PathBuf}, sync::{Mutex, OnceLock}};

pub use semio_framework_print::*;

pub fn product_directory() -> &'static Path {
    static DIRECTORY: OnceLock<PathBuf> = OnceLock::new();
    DIRECTORY.get_or_init(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../.."))
}

fn leaked(relative: &str) -> &'static [u8] {
    static READ: OnceLock<Mutex<HashMap<String, &'static [u8]>>> = OnceLock::new();
    let mut read = READ.get_or_init(Default::default).lock().unwrap();
    read.entry(relative.to_owned()).or_insert_with(|| {
        let path = product_directory().join(relative);
        Box::leak(std::fs::read(&path).unwrap_or_else(|error| panic!("print test input {}: {error}", path.display())).into_boxed_slice())
    })
}

pub fn text(relative: &str) -> &'static str {
    std::str::from_utf8(leaked(relative)).unwrap_or_else(|error| panic!("print test input {relative} is not UTF-8: {error}"))
}

pub fn bytes(relative: &str) -> &'static [u8] {
    leaked(relative)
}

#[path = "../🧬️chart-mutations/🦀️.rs"]
mod tests;
