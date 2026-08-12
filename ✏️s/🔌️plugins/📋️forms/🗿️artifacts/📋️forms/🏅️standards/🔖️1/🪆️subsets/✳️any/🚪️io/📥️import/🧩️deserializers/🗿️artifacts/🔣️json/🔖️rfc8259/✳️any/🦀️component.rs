//! Deserialize forms via stdio.json.
use crate::artifacts::forms::FormsSnapshot;
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::write_json_text;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<FormsSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let text = write_json_text(&from.value);
    serde_json::from_str(&text).map_err(|e| store::TextError::new(format!("forms<-json: {e}"), dsl::TextSpan::at(1, 1)))
}

pub fn deserialize_text(text: &str) -> Result<FormsSnapshot, store::TextError> {
    <FormsSnapshot as store::ArtifactDsl>::parse_dsl(text)
}
