//! 🔺️ Sparse diff construction for the `rename-activity` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏃activities` per Wave C.

use super::mutation::RenameActivity;
use crate::artifacts::program::diff::{ProgramActivitiesDelta, ProgramActivitiesPatchEntry};
use crate::artifacts::program::registers::ActivityPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameActivity, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.activities.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No activity exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This activity already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = ActivityPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { activities: Some(ProgramActivitiesDelta { patched: vec![ProgramActivitiesPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
