//! 🚪️ presentation <- md — foreign `Deserializer<PresentationSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Degenerate placeholder
//! (unchanged behaviour, pre-dates this ticket): reads the DSL text back out of the single
//! `Paragraph`/`Text` block the paired export leaf wrote, rather than a real markdown->presentation
//! semantic mapping (out of scope here). Still wraps the full `.presentation` DSL text losslessly in
//! that one block, so this hop is `IoFidelity::Canonical`, not `Lossy`.

use crate::artifacts::presentation::PresentationSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::md::schema::snapshot::{MdBlock, MdInline};
use semio_s_plugin_stdio::artifacts::md::MdSnapshot;

pub const MD_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId::ANY };

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

pub struct MdIntoPresentation;

impl Deserializer<PresentationSnapshot> for MdIntoPresentation {
    const FROM: Dialect = MD_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Canonical;
    async fn deserialize(payload: &IoPayload) -> IoResult<PresentationSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "MdIntoPresentation: expected a binary md payload".to_string(), diagnostics: Vec::new() });
        };
        let md = <MdSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| IoError { message: format!("MdIntoPresentation: {error}"), diagnostics: Vec::new() })?;
        let snapshot = <PresentationSnapshot as store::ArtifactDsl>::parse_dsl(&extract_placeholder_text(&md)).map_err(|error| IoError { message: format!("MdIntoPresentation: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(snapshot))
    }
}
