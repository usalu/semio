//! gisterrain <- json
use crate::artifacts::gisterrain::{GisTerrainSnapshot};
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::json::standards::v_rfc8259::subsets::any::schema::snapshot::{JsonMember, JsonValue};

pub fn register() {}

/// 🌉️ `JsonSnapshot.value` is stdio's own `JsonValue` (never `serde_json::Value` — see that
/// type's own doc comment). Both directions live here since this leaf needs both. Mirrors the
/// sibling export leaf's own copy (per-leaf convention).
fn json_value_to_serde(value: &JsonValue) -> serde_json::Value {
    match value {
        JsonValue::Null => serde_json::Value::Null,
        JsonValue::Bool { value } => serde_json::Value::Bool(*value),
        JsonValue::Number { lexeme } => serde_json::from_str(lexeme).unwrap_or(serde_json::Value::Null),
        JsonValue::String { value } => serde_json::Value::String(value.clone()),
        JsonValue::Array { items } => serde_json::Value::Array(items.iter().map(json_value_to_serde).collect()),
        JsonValue::Object { members } => serde_json::Value::Object(members.iter().map(|member| (member.key.clone(), json_value_to_serde(&member.value))).collect()),
    }
}

fn serde_value_to_json(value: &serde_json::Value) -> JsonValue {
    match value {
        serde_json::Value::Null => JsonValue::Null,
        serde_json::Value::Bool(value) => JsonValue::Bool { value: *value },
        serde_json::Value::Number(number) => JsonValue::Number { lexeme: number.to_string() },
        serde_json::Value::String(value) => JsonValue::String { value: value.clone() },
        serde_json::Value::Array(items) => JsonValue::Array { items: items.iter().map(serde_value_to_json).collect() },
        serde_json::Value::Object(members) => JsonValue::Object { members: members.iter().map(|(key, value)| JsonMember { key: key.clone(), value: serde_value_to_json(value) }).collect() },
    }
}

pub fn deserialize(from: &JsonSnapshot) -> Result<GisTerrainSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let snap: GisTerrainSnapshot = serde_json::from_value(from.to_serde_value())
        .map_err(|e| store::TextError::new(format!("gisterrain<-json: {e}"), dsl::TextSpan::at(1, 1)))?;

    Ok(snap)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<GisTerrainSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let raw: serde_json::Value = serde_json::from_str(text).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    deserialize(&JsonSnapshot::from_value(raw))
}
