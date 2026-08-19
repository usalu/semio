//! Deserialize flow via stdio.md.
use crate::artifacts::flow::FlowSnapshot;
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn deserialize(from: &MdSnapshot) -> Result<FlowSnapshot, store::TextError> {
    let _ = STDIO_MD_DOCUMENT_SCHEMA;
    <FlowSnapshot as store::ArtifactDsl>::parse_dsl(&from.to_text())
}

pub async fn deserialize_text(text: &str) -> Result<FlowSnapshot, store::TextError> {
    <FlowSnapshot as store::ArtifactDsl>::parse_dsl(text)
}
