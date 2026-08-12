//! Deserialize writer via stdio.json.
use crate::artifacts::writer::WriterSnapshot;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<WriterSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    serde_json::from_value(from.to_serde_value()).map_err(|e| store::TextError::new(format!("writer<-json: {e}"), dsl::TextSpan::at(1, 1)))
}

pub fn deserialize_text(text: &str) -> Result<WriterSnapshot, store::TextError> {
    <WriterSnapshot as store::ArtifactDsl>::parse_dsl(text)
}
