//! 🔺️ Sparse diff construction for the `replace-conflict` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⚔️conflicts` per Wave C.

use super::ReplaceConflict;
use crate::diff::ProgramConflictsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceConflict, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.conflict.header.id;
    let Some(position) = base.conflicts.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No conflict exists with this id.", [id.0.clone()]);
    };
    if base.conflicts[position] == payload.conflict {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This conflict already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramConflictsDelta::removal(&base.conflicts, position);
    delta.absorb(ProgramConflictsDelta::insertion(position, payload.conflict.clone()));
    protocol::MutationOutcome::new(ProgramDiff { conflicts: Some(delta), ..Default::default() })
}
