//! 🔺️ Sparse diff construction for the `replace-conflict` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⚔️conflicts` per Wave C.

use super::ReplaceConflict;
use crate::artifacts::program::diff::{ProgramConflictsDelta, ProgramConflictsPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceConflict, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.conflicts.iter().find(|row| row.header.id == payload.conflict.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No conflict exists with this id.", [payload.conflict.header.id.0.clone()]);
    };
    if existing == &payload.conflict {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This conflict already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.conflict).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { conflicts: Some(ProgramConflictsDelta { patched: vec![ProgramConflictsPatchEntry { id: payload.conflict.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
