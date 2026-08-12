//! present -> md
//!
//! 🩹️ `stdio_gap`/foreign-lag fix (not part of this wave's video/gif/html/typst/svg-dwg scope):
//! `MdSnapshot`'s `body: String` field was replaced by a real `blocks: Vec<MdBlock>` tree by a
//! concurrent stdio wave (commit `ad0fc0019b`, landed mid-session per `git log`), breaking this
//! pre-existing degenerate placeholder leaf's compile. Fixed as a minimal lagging-call-site
//! update — still the same degenerate "whole DSL text, unparsed" placeholder as before (a single
//! `Paragraph`/`Text` block), not a real present->markdown semantic mapping (out of this leaf's
//! and this wave's scope).
use crate::artifacts::present::schema::snapshot::PresentSnapshot;
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::md::schema::snapshot::{MdBlock, MdInline};

pub fn register() {}

pub fn serialize(snapshot: &PresentSnapshot) -> Result<MdSnapshot, store::TextError> {
    let text = <PresentSnapshot as store::ArtifactDsl>::print_dsl(snapshot);
    Ok(MdSnapshot {
        schema: STDIO_MD_DOCUMENT_SCHEMA.into(),
        blocks: vec![MdBlock::Paragraph { inlines: vec![MdInline::Text { text }] }],
    })
}

pub fn serialize_bytes(snapshot: &PresentSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<MdSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}
