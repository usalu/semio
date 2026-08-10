//! ser json to txt
use crate::artifacts::json::JsonSnapshot;
use crate::artifacts::txt::{TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA};
pub fn register() {}
pub fn serialize(from: &JsonSnapshot) -> Result<TxtSnapshot, store::PackError> {
    let text = serde_json::to_string_pretty(&from.value).map_err(|e| store::PackError::Schema(e.to_string()))?;
    Ok(TxtSnapshot { schema: STDIO_TXT_DOCUMENT_SCHEMA.into(), text })
}
pub fn serialize_text(from: &JsonSnapshot) -> Result<String, store::PackError> {
    Ok(store::DocumentDsl::print_dsl(&serialize(from)?))
}
