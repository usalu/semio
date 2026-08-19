//! Serialize flow to stdio.md.
use crate::artifacts::flow::FlowSnapshot;
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn serialize(from: &FlowSnapshot) -> Result<MdSnapshot, store::PackError> {
    let _ = STDIO_MD_DOCUMENT_SCHEMA;
    Ok(MdSnapshot::from_text(&<FlowSnapshot as store::ArtifactDsl>::print_dsl(from)))
}

pub async fn serialize_text(from: &FlowSnapshot) -> Result<String, store::PackError> {
    Ok(<FlowSnapshot as store::ArtifactDsl>::print_dsl(from))
}
