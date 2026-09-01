//! 🔺️ Sparse diff construction for the `delete-document` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📄documents` per Wave C.

use super::DeleteDocument;
use crate::artifacts::program::diff::ProgramArtifactsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub async fn diff(payload: &DeleteDocument, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.artifacts.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No document exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { documents: Some(ProgramArtifactsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
