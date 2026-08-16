//! 🔺️ Sparse diff construction for the `create-document` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📄documents` per Wave C.

use super::mutation::CreateDocument;
use crate::artifacts::program::diff::ProgramArtifactsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub fn diff(payload: &CreateDocument, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.document.header.id.clone();
    if base.artifacts.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A document already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { documents: Some(ProgramArtifactsDelta { added: vec![payload.document.clone()], ..Default::default() }), ..Default::default() })
}
