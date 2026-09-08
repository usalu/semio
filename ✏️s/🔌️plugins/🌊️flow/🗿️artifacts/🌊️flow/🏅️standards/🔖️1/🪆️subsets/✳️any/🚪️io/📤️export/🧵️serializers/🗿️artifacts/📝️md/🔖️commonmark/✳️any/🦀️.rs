//! Serialize flow to stdio.md.
use crate::FlowSnapshot;
use semio_s_artifact_stdio_md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(from: &FlowSnapshot) -> Result<MdSnapshot, store::PackError> {
    let _ = STDIO_MD_DOCUMENT_SCHEMA;
    Ok(MdSnapshot::from_text(&<FlowSnapshot as store::ArtifactDsl>::print_dsl(from)))
}

pub fn serialize_text(from: &FlowSnapshot) -> Result<String, store::PackError> {
    Ok(<FlowSnapshot as store::ArtifactDsl>::print_dsl(from))
}
