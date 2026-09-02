//! Deserialize flow via stdio.json.
use crate::artifacts::flow::FlowSnapshot;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<FlowSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let dsl_value: dsl::DslValue = from.to_serde_value().into();
    dsl::FromValue::from_value(dsl_value).map_err(|e| store::TextError::new(format!("flow<-json: {e}"), dsl::TextSpan::at(1, 1)))
}

pub fn deserialize_text(text: &str) -> Result<FlowSnapshot, store::TextError> {
    <FlowSnapshot as store::ArtifactDsl>::parse_dsl(text)
}
