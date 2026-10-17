//! 🥣️ Companion tests of value: retirement integration, refusal codecs, typed values, paged and shared text, leases, bounded and retained cloning.
//!
//! 🧭️ The root laws are mounted flat; the laws of each value module sit below a wrapper of that module that restates its imports and uses the crate's `testkit` hooks.
#![cfg(test)]

pub use semio_framework_value::*;
use semio_framework_value_testing_allocator as allocation_testing;
pub(crate) use allocation_testing::observe_retirement_allocations;

use std::{collections::HashMap, path::{Path, PathBuf}, sync::{Mutex, OnceLock}};

pub fn product_directory() -> &'static Path {
    static DIRECTORY: OnceLock<PathBuf> = OnceLock::new();
    DIRECTORY.get_or_init(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../.."))
}

fn leaked(relative: &str) -> &'static [u8] {
    static READ: OnceLock<Mutex<HashMap<String, &'static [u8]>>> = OnceLock::new();
    let mut read = READ.get_or_init(Default::default).lock().unwrap();
    read.entry(relative.to_owned()).or_insert_with(|| {
        let path = product_directory().join(relative);
        Box::leak(std::fs::read(&path).unwrap_or_else(|error| panic!("test input {}: {error}", path.display())).into_boxed_slice())
    })
}

pub fn text(relative: &str) -> &'static str {
    std::str::from_utf8(leaked(relative)).unwrap_or_else(|error| panic!("test input {relative} is not UTF-8: {error}"))
}

pub fn bytes(relative: &str) -> &'static [u8] {
    leaked(relative)
}

#[path = "../../♻️retirement/🧪️tests/🔬️unit/🦀️.rs"]
mod original_retirement_integration_tests;

#[path = "../../⚠️refusal/🧪️tests/🦀️.rs"]
mod refusal_tests;


#[path = "📦️paged/🦀️.rs"]
pub mod paged;

#[path = "🎮️paged-append/🦀️.rs"]
pub mod paged_append;

#[path = "📝️shared-utf8/🦀️.rs"]
pub mod shared_utf8;


#[path = "🧬️bounded-clone/🦀️.rs"]
pub mod bounded_clone;

#[path = "🔗️read/🦀️.rs"]
pub mod read;



#[path = "🌱️retained-clone/🦀️.rs"]
pub mod retained_clone;

