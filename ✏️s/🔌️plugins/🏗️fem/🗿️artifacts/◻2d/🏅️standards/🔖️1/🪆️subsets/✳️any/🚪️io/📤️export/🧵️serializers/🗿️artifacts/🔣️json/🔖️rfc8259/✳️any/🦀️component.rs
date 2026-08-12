//! fem2d -> json. `stdio.json`'s real `JsonSnapshot` shape (`value: JsonValue`, a lexeme-
//! preserving custom tree, not `serde_json::Value`) landed after this leaf was first written —
//! lagging call site fixed to match (ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-
//! MEDIA-FORMAT-RETIREMENT W5a): `serde_json::to_value(snapshot)` still produces the real
//! structured JSON tree (every `Fem2dSnapshot` field, not a single blob like the csv/md leaves),
//! walked into the target `JsonValue` shape by `serde_to_json_value`; `serialize_bytes` writes it
//! through stdio's own real RFC 8259 text codec (`write_json_text`), not a re-derived encoder.
use crate::artifacts::fem2d::Fem2dSnapshot;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::{write_json_text, JsonMember, JsonValue};

pub fn register() {}

//#region 🔖️SerdeBridge
/// 🌉️ `serde_json::Value` -> stdio's real `JsonValue` tree — structural, lossless walk.
fn serde_to_json_value(v: &serde_json::Value) -> JsonValue {
    match v {
        serde_json::Value::Null => JsonValue::Null,
        serde_json::Value::Bool(value) => JsonValue::Bool { value: *value },
        serde_json::Value::Number(n) => JsonValue::Number { lexeme: n.to_string() },
        serde_json::Value::String(value) => JsonValue::String { value: value.clone() },
        serde_json::Value::Array(items) => JsonValue::Array { items: items.iter().map(serde_to_json_value).collect() },
        serde_json::Value::Object(members) => JsonValue::Object { members: members.iter().map(|(key, value)| JsonMember { key: key.clone(), value: serde_to_json_value(value) }).collect() },
    }
}
//#endregion 🔖️SerdeBridge

pub fn serialize(snapshot: &Fem2dSnapshot) -> Result<JsonSnapshot, store::TextError> {
    let raw = serde_json::to_value(snapshot).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    Ok(JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value: serde_to_json_value(&raw) })
}

pub fn serialize_bytes(snapshot: &Fem2dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(write_json_text(&serialize(snapshot)?.value).into_bytes())
}
