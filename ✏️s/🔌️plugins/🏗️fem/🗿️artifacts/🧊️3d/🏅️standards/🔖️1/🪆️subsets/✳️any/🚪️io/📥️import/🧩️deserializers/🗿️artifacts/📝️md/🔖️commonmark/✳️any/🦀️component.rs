//! fem3d <- md
use crate::artifacts::fem3d::Fem3dSnapshot;
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &MdSnapshot) -> Result<Fem3dSnapshot, store::TextError> {
    let _ = STDIO_MD_DOCUMENT_SCHEMA;
    <Fem3dSnapshot as store::ArtifactDsl>::parse_dsl(&from.body)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Fem3dSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    <Fem3dSnapshot as store::ArtifactDsl>::parse_dsl(text)
}
