//! 🥣️ Companion tests of pack-json: ordered values, native canonical JSON trees, text cursors, wire cursors, read policy, refusal receipts and byte-exact strings.
//!
//! 🧭️ The unit laws are mounted flat, so `super::*` resolves to the public vocabulary of the crate; the laws of the private encode and decode modules sit below wrappers that restate those modules' imports.
#![cfg(test)]

pub use semio_framework_pack_json::*;
pub use semio_framework_pack_json::testkit::JsonLexeme;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value::{DslValue, ValueError, ValueRefusalKind};

#[global_allocator]
static TEST_ALLOCATION_OBSERVER: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;

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

//#region 🔖️Tests
#[path = "../🔬️unit/🦀️.rs"]
mod tests;
#[path = "../🧩️members/🦀️.rs"]
mod member_tests;
//#endregion 🔖️Tests

#[path = "../⚠️refusal/🦀️.rs"]
mod refusal_tests;

#[path = "../../📥️decode/🛂️policy/🧪️tests/🦀️.rs"]
mod read_policy_tests;

#[path = "../../📥️decode/🧾️receipt/🧪️tests/🦀️.rs"]
mod normal_refusal_receipt_tests;

#[path = "../../📥️decode/🔣️string/📦️bytes/🧪️tests/🦀️.rs"]
mod original_string_byte_tests;


#[path = "🧵️operation/🦀️.rs"]
pub mod native_operation;

#[path = "🔤️text/🦀️.rs"]
pub mod native_text;

#[path = "🧭️tree/🦀️.rs"]
pub mod native_tree;

#[path = "📨️wire/🦀️.rs"]
pub mod original_wire;


#[test]
fn owned_json_member_transfer_keeps_independent_key_value_and_backing_custody() {
    let mut key = String::with_capacity(128);
    key.push_str("second");
    let key_pointer = key.as_ptr();
    let mut object = Object::new();
    object.insert("first", Value::Bool(true));
    object.insert(key, Value::String("payload".into()));
    let independent: serde_json::Value = serde_json::from_str(r#"{"first":true,"second":"payload"}"#).expect("independent object");
    assert_eq!(object.last_member().expect("member").0.capacity(), 128);
    let backing = object.member_storage_bytes();
    let (key, value) = object.pop_member().expect("owned member");
    assert_eq!(key.as_ptr(), key_pointer);
    assert_eq!(value.as_str(), independent[&key].as_str());
    assert_eq!(object.member_storage_bytes(), backing);
    assert_eq!(object.pop_member().expect("first member").1.as_bool(), independent["first"].as_bool());
    assert!(object.last_member().is_none());
    assert_eq!(object.member_storage_bytes(), backing);
}
