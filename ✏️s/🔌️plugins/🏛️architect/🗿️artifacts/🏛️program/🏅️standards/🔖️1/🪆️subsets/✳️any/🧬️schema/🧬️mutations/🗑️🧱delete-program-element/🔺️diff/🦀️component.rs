//! 🔺️ Sparse diff construction for the `delete-program-element` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧱elements` per Wave C.

use super::mutation::DeleteProgramElement;
use crate::artifacts::program::diff::ProgramElementsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub async fn diff(payload: &DeleteProgramElement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.elements.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No program element exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { elements: Some(ProgramElementsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
