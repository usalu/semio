//! 🔺️ Sparse diff construction for the `replace-activity` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏃activities` per Wave C.

use super::ReplaceActivity;
use crate::diff::ProgramActivitiesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns:
/// `removed = [id]`, `added = [payload row]`, and `reordered` (the base order) unless the row was last, so the new row keeps its position.
pub fn diff(payload: &ReplaceActivity, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.activity.header.id;
    let Some(position) = base.activities.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No activity exists with this id.", [id.0.clone()]);
    };
    if base.activities[position] == payload.activity {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This activity already matches the requested value.").at([id.0.clone()])]);
    }
    let reordered = (position + 1 != base.activities.len()).then(|| base.activities.iter().map(|row| row.header.id.0.clone()).collect());
    protocol::MutationOutcome::new(ProgramDiff { activities: Some(ProgramActivitiesDelta { removed: vec![id.0.clone()], added: vec![payload.activity.clone()], reordered, ..Default::default() }), ..Default::default() })
}
