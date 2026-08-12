//! procedural3d -> json
//!
//! 🩹️ w5b-close fix (stdio_gap/foreign-lag, not svg/dwg-pattern scope — the deletion task itself
//! never touched this file; see w5b-close-report.md): `JsonSnapshot.value` was retyped from
//! `serde_json::Value` to stdio's own lexeme-preserving `JsonValue` by a concurrent stdio wave,
//! breaking this pre-existing leaf's compile (matches procedural's own W5b report's stdio_gaps
//! finding). Mirrors 🗒️note's/🎥️shooting's identical fix: a real structural `serde_json::Value ->
//! JsonValue` converter plus stdio's own real `write_json_pretty`.
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::{write_json_pretty, JsonMember, JsonSnapshot, JsonValue};
use semio_s_plugin_stdio::artifacts::json::STDIO_JSON_DOCUMENT_SCHEMA;

pub fn register() {}

/// 🔁️ Structural `serde_json::Value -> JsonValue` conversion (stdio's own `JsonValue` has no
/// built-in bridge to `serde_json::Value` — see this file's module doc comment).
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

pub fn serialize(snapshot: &Procedural3dSnapshot) -> Result<JsonSnapshot, store::TextError> {
    let value = serde_json::to_value(snapshot).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    Ok(JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value: serde_to_json_value(&value) })
}

pub fn serialize_bytes(snapshot: &Procedural3dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(write_json_pretty(&serialize(snapshot)?.value).into_bytes())
}
