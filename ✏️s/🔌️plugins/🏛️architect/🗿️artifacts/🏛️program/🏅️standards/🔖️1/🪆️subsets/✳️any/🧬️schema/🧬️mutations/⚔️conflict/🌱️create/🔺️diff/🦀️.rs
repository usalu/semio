//! 🔺️ Sparse diff construction for the `create-conflict` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⚔️conflicts` per Wave C.

use super::CreateConflict;
use crate::diff::ProgramConflictsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub fn diff(payload: &CreateConflict, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.conflict.header.id.clone();
    if base.conflicts.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A conflict already exists with this id.", [id.0]);
    }
    protocol::MutationOutcome::new(ProgramDiff { conflicts: Some(ProgramConflictsDelta { added: vec![payload.conflict.clone()], ..Default::default() }), ..Default::default() })
}
