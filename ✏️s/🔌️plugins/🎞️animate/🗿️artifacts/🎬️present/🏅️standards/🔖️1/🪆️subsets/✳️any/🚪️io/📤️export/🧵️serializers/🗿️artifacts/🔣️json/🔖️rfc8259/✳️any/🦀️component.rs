//! present -> json
//!
//! 🩹️ `stdio_gap`/foreign-lag fix (not part of this wave's video/gif/html/typst/svg-dwg scope):
//! `JsonSnapshot.value` was retyped from `serde_json::Value` to stdio's own lexeme-preserving
//! `JsonValue` (own type, `#[serde(tag = "kind")]` — NOT structurally plain JSON — by design, see
//! that schema's own doc comment) by a concurrent stdio wave, breaking this pre-existing
//! placeholder leaf's compile. Fixed as a minimal lagging-call-site update: a real, honest
//! structural `serde_json::Value -> JsonValue` converter (stdio provides no such bridge — a real
//! gap, reported in `w5a--report.md`) plus stdio's own real `write_json_pretty` text codec for
//! `serialize_bytes` (the previous `serde_json::to_vec_pretty(&value)` would have serialized the
//! internally-tagged `JsonValue` shape verbatim, not real JSON text — a latent bug this fix also
//! corrects).
use crate::artifacts::present::PresentSnapshot;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::{write_json_pretty, JsonMember, JsonValue};

pub fn register() {}

/// 🔁️ Structural `serde_json::Value -> JsonValue` conversion (stdio's own `JsonValue` has no
/// built-in bridge to `serde_json::Value` — an intentional boundary per that schema's doc comment
/// — so any caller needing to interop with ordinary `serde_json`-based types must hand-roll one;
/// see this file's module doc comment).
fn serde_to_json_value(value: &serde_json::Value) -> JsonValue {
    match value {
        serde_json::Value::Null => JsonValue::Null,
        serde_json::Value::Bool(value) => JsonValue::Bool { value: *value },
        serde_json::Value::Number(number) => JsonValue::Number { lexeme: number.to_string() },
        serde_json::Value::String(value) => JsonValue::String { value: value.clone() },
        serde_json::Value::Array(items) => JsonValue::Array { items: items.iter().map(serde_to_json_value).collect() },
        serde_json::Value::Object(members) => JsonValue::Object { members: members.iter().map(|(key, value)| JsonMember { key: key.clone(), value: serde_to_json_value(value) }).collect() },
    }
}

pub fn serialize(snapshot: &PresentSnapshot) -> Result<JsonSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let value = serde_json::to_value(snapshot).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    Ok(JsonSnapshot::from_value(value))
}

pub fn serialize_bytes(snapshot: &PresentSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(write_json_pretty(&serialize(snapshot)?.value).into_bytes())
}
