//! fem3d <- json. `stdio.json`'s real `JsonSnapshot` shape (`value: JsonValue`, a lexeme-
//! preserving custom tree, not `serde_json::Value`) landed after this leaf was first written —
//! lagging call site fixed to match (ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-
//! MEDIA-FORMAT-RETIREMENT W5a): `json_value_to_serde` walks the real `JsonValue` tree back into
//! `serde_json::Value` so `serde_json::from_value` still works; `deserialize_bytes` parses through
//! stdio's own real RFC 8259 text codec (`parse_json_text`), not a re-derived parser.
use crate::artifacts::fem3d::Fem3dSnapshot;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::{parse_json_text, JsonValue};

pub fn register() {}

//#region 🔖️SerdeBridge
/// 🌉️ stdio's real `JsonValue` tree -> `serde_json::Value` — structural, lossless walk (mirror
/// of the sibling exporter's `serde_to_json_value`).
fn json_value_to_serde(v: &JsonValue) -> serde_json::Value {
    match v {
        JsonValue::Null => serde_json::Value::Null,
        JsonValue::Bool { value } => serde_json::Value::Bool(*value),
        JsonValue::Number { lexeme } => serde_json::from_str(lexeme).unwrap_or(serde_json::Value::Null),
        JsonValue::String { value } => serde_json::Value::String(value.clone()),
        JsonValue::Array { items } => serde_json::Value::Array(items.iter().map(json_value_to_serde).collect()),
        JsonValue::Object { members } => serde_json::Value::Object(members.iter().map(|m| (m.key.clone(), json_value_to_serde(&m.value))).collect()),
    }
}
//#endregion 🔖️SerdeBridge

pub fn deserialize(from: &JsonSnapshot) -> Result<Fem3dSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let raw = json_value_to_serde(&from.value);
    let snap: Fem3dSnapshot = serde_json::from_value(raw)
        .map_err(|e| store::TextError::new(format!("fem3d<-json: {e}"), dsl::TextSpan::at(1, 1)))?;
    Ok(snap)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Fem3dSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let value = parse_json_text(text)?;
    deserialize(&JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value })
}
