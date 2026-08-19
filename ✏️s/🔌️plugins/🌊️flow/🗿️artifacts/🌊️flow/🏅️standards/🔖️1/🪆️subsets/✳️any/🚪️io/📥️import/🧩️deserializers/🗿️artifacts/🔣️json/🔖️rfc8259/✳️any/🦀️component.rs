//! Deserialize flow via stdio.json.
use crate::artifacts::flow::FlowSnapshot;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn deserialize(from: &JsonSnapshot) -> Result<FlowSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    serde_json::from_value(from.to_serde_value()).map_err(|e| store::TextError::new(format!("flow<-json: {e}"), dsl::TextSpan::at(1, 1)))
}

pub async fn deserialize_text(text: &str) -> Result<FlowSnapshot, store::TextError> {
    <FlowSnapshot as store::ArtifactDsl>::parse_dsl(text)
}
