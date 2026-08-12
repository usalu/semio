//! present <- md
//!
//! 🩹️ `stdio_gap`/foreign-lag fix — see the paired export leaf's doc comment (same wave,
//! `MdSnapshot.body: String` -> `blocks: Vec<MdBlock>`). Mirrors the export leaf's degenerate
//! placeholder exactly: reads the DSL text back out of the single `Paragraph`/`Text` block the
//! export leaf wrote, rather than a real markdown->present semantic mapping (out of scope here).
use crate::artifacts::present::schema::snapshot::PresentSnapshot;
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::md::schema::snapshot::{MdBlock, MdInline};

pub fn register() {}

fn extract_placeholder_text(from: &MdSnapshot) -> String {
    for block in &from.blocks {
        if let MdBlock::Paragraph { inlines } = block {
            for inline in inlines {
                if let MdInline::Text { text } = inline {
                    return text.clone();
                }
            }
        }
    }
    String::new()
}

pub fn deserialize(from: &MdSnapshot) -> Result<PresentSnapshot, store::TextError> {
    let _ = STDIO_MD_DOCUMENT_SCHEMA;
    <PresentSnapshot as store::ArtifactDsl>::parse_dsl(&extract_placeholder_text(from))
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<PresentSnapshot, store::TextError> {
    let md = <MdSnapshot as store::ArtifactPack>::decode_pack(bytes)
        .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    deserialize(&md)
}
