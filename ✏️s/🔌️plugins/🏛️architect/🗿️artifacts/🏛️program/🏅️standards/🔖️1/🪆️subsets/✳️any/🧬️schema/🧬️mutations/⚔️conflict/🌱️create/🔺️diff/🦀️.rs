//! 🔺️ Sparse diff construction for the `create-conflict` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⚔️conflicts` per Wave C.

use super::CreateConflict;
use crate::diff::ProgramConflictsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateConflict, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.conflict.header.id;
    if base.conflicts.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A conflict already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.conflicts.len());
    if at > base.conflicts.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the conflict list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { conflicts: Some(ProgramConflictsDelta::insertion(at, payload.conflict.clone())), ..Default::default() })
}
