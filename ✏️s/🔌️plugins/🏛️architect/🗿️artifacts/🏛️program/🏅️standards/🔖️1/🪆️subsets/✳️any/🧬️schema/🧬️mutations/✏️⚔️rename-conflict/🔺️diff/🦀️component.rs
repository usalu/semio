//! 🔺️ Sparse diff construction for the `rename-conflict` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⚔️conflicts` per Wave C.

use super::mutation::RenameConflict;
use crate::artifacts::program::diff::{ProgramConflictsDelta, ProgramConflictsPatchEntry};
use crate::artifacts::program::registers::ConflictPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameConflict, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.conflicts.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No conflict exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This conflict already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = ConflictPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { conflicts: Some(ProgramConflictsDelta { patched: vec![ProgramConflictsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
