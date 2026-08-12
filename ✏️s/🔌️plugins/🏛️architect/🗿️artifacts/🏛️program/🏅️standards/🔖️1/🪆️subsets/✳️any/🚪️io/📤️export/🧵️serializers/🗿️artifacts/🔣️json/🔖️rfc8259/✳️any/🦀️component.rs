//! program -> json
//!
//! 🩹️ `stdio_gap`/foreign-lag fix (not part of this wave's csv/tsv scope): `JsonSnapshot.value`
//! was retyped from `serde_json::Value` to stdio's own lexeme-preserving `JsonValue`
//! (`#[serde(tag = "kind")]`, NOT structurally plain JSON by design) by a concurrent stdio wave,
//! breaking this pre-existing placeholder leaf's compile. Fixed as a minimal lagging-call-site
//! update, mirroring the same pattern animate/fem used for the identical gap: a real, honest
//! structural `serde_json::Value -> JsonValue` converter (stdio provides no such bridge) plus
//! stdio's own real `write_json_pretty` text codec for `serialize_bytes`.
use crate::artifacts::program::ProgramSnapshot;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::{write_json_pretty, JsonMember, JsonValue};

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

pub fn serialize(snapshot: &ProgramSnapshot) -> Result<JsonSnapshot, store::TextError> {
    let value = serde_json::to_value(snapshot).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    Ok(JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value: serde_to_json_value(&value) })
}

pub fn serialize_bytes(snapshot: &ProgramSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(write_json_pretty(&serialize(snapshot)?.value).into_bytes())
}
