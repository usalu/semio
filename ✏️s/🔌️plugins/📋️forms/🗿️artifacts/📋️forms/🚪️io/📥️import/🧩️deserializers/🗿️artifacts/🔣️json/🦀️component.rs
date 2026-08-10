//! forms <- json
use crate::artifacts::forms::{FormsSnapshot, FORMS_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};
pub fn register() {}
pub fn deserialize(from: &JsonSnapshot) -> Result<FormsSnapshot, String> {
    let mut snap: FormsSnapshot = serde_json::from_value(from.value.clone()).map_err(|e| e.to_string())?;
    if snap.schema.is_empty() { snap.schema = FORMS_DOCUMENT_SCHEMA.into(); }
    Ok(snap)
}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<FormsSnapshot, String> {
    let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
    let value: serde_json::Value = serde_json::from_str(text).map_err(|e| e.to_string())?;
    deserialize(&JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value })
}
