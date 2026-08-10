//! Deserialize mathematical via stdio.md.
use crate::artifacts::mathematical::MathematicalSnapshot;
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &MdSnapshot) -> Result<MathematicalSnapshot, store::TextError> {
    let _ = STDIO_MD_DOCUMENT_SCHEMA;
    <MathematicalSnapshot as store::ArtifactDsl>::parse_dsl(&from.body)
}

pub fn deserialize_text(text: &str) -> Result<MathematicalSnapshot, store::TextError> {
    <MathematicalSnapshot as store::ArtifactDsl>::parse_dsl(text)
}
