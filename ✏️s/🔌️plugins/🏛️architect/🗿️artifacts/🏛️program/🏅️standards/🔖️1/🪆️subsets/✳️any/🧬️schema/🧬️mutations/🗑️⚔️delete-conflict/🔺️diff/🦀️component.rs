//! 🔺️ Sparse diff construction for the `delete-conflict` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⚔️conflicts` per Wave C.

use super::mutation::DeleteConflict;
use crate::artifacts::program::diff::ProgramConflictsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub fn diff(payload: &DeleteConflict, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.conflicts.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No conflict exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { conflicts: Some(ProgramConflictsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
