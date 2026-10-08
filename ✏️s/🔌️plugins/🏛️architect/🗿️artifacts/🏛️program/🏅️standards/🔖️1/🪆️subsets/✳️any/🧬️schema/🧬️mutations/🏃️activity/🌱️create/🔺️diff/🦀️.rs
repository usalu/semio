//! 🔺️ Sparse diff construction for the `create-activity` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏃activities` per Wave C.

use super::CreateActivity;
use crate::diff::ProgramActivitiesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateActivity, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.activity.header.id;
    if base.activities.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "An activity already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.activities.len());
    if at > base.activities.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the activity list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { activities: Some(ProgramActivitiesDelta::insertion(at, payload.activity.clone())), ..Default::default() })
}
