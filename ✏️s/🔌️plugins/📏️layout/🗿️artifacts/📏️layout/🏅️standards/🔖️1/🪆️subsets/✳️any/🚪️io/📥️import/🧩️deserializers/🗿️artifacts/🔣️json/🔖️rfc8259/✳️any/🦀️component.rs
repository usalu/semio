//! Deserialize layout via stdio.json.
use crate::artifacts::layout::LayoutSnapshot;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<LayoutSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    serde_json::from_value(from.value.clone()).map_err(|e| store::TextError::new(format!("layout<-json: {e}"), dsl::TextSpan::at(1, 1)))
}

pub fn deserialize_text(text: &str) -> Result<LayoutSnapshot, store::TextError> {
    <LayoutSnapshot as store::ArtifactDsl>::parse_dsl(text)
}
