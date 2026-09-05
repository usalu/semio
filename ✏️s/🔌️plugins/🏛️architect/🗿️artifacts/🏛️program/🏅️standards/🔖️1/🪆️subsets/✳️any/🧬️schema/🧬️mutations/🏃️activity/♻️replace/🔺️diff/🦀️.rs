//! 🔺️ Sparse diff construction for the `replace-activity` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏃activities` per Wave C.

use super::ReplaceActivity;
use crate::artifacts::program::diff::{ProgramActivitiesDelta, ProgramActivitiesPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceActivity, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.activities.iter().find(|row| row.header.id == payload.activity.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No activity exists with this id.", [payload.activity.header.id.0.clone()]);
    };
    if existing == &payload.activity {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This activity already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.activity).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { activities: Some(ProgramActivitiesDelta { patched: vec![ProgramActivitiesPatchEntry { id: payload.activity.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
