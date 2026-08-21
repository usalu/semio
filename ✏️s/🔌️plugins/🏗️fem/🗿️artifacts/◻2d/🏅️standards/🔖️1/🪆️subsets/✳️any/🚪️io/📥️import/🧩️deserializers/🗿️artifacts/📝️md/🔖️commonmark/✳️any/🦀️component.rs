//! fem2d <- md. `stdio.md`'s real `MdSnapshot` shape (`blocks: Vec<MdBlock>`) landed after this
//! leaf was first written — lagging call site fixed to match (ticket 26/08/11/SEMIO-ARTIFACT-
//! UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W5a): reads the DSL text back out of the
//! first `CodeBlock` (mirror of the sibling exporter's encoding).
use crate::artifacts::fem2d::Fem2dSnapshot;
use semio_s_plugin_stdio::artifacts::md::schema::snapshot::MdBlock;
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn deserialize(from: &MdSnapshot) -> Result<Fem2dSnapshot, store::TextError> {
    let _ = STDIO_MD_DOCUMENT_SCHEMA;
    let literal = from
        .blocks
        .iter()
        .find_map(|b| match b {
            MdBlock::CodeBlock { literal, .. } => Some(literal.as_str()),
            _ => None,
        })
        .ok_or_else(|| store::TextError::new("fem2d <- md: no code block found", dsl::TextSpan::at(1, 1)))?;
    <Fem2dSnapshot as store::ArtifactDsl>::parse_dsl(literal)
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<Fem2dSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    <Fem2dSnapshot as store::ArtifactDsl>::parse_dsl(text)
}
