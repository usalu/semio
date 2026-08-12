//! Serialize mathematical to stdio.md.
use crate::artifacts::mathematical::MathematicalSnapshot;
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(from: &MathematicalSnapshot) -> Result<MdSnapshot, store::PackError> {
    let _ = STDIO_MD_DOCUMENT_SCHEMA;
    Ok(MdSnapshot::from_text(&<MathematicalSnapshot as store::ArtifactDsl>::print_dsl(from)))
}

pub fn serialize_text(from: &MathematicalSnapshot) -> Result<String, store::PackError> {
    Ok(<MathematicalSnapshot as store::ArtifactDsl>::print_dsl(from))
}
