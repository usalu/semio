//! fem2d -> md. `stdio.md`'s real `MdSnapshot` shape (`blocks: Vec<MdBlock>`, real CommonMark
//! block tree) landed after this leaf was first written — lagging call site fixed to match
//! (ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W5a): the
//! DSL text is wrapped in one `CodeBlock` (verbatim `literal`, no markdown-escaping risk to the
//! payload), same single-blob-payload shape as before.
use crate::artifacts::fem2d::Fem2dSnapshot;
use semio_s_plugin_stdio::artifacts::md::schema::snapshot::MdBlock;
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn serialize(snapshot: &Fem2dSnapshot) -> Result<MdSnapshot, store::TextError> {
    Ok(MdSnapshot { schema: STDIO_MD_DOCUMENT_SCHEMA.into(), blocks: vec![MdBlock::CodeBlock { info: None, literal: <Fem2dSnapshot as store::ArtifactDsl>::print_dsl(snapshot) }] })
}

pub async fn serialize_bytes(snapshot: &Fem2dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<Fem2dSnapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())
}
