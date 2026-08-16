//! 🔺️ Sparse diff construction for the `create-activity` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏃activities` per Wave C.

use super::mutation::CreateActivity;
use crate::artifacts::program::diff::ProgramActivitiesDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub fn diff(payload: &CreateActivity, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.activity.header.id.clone();
    if base.activities.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "An activity already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { activities: Some(ProgramActivitiesDelta { added: vec![payload.activity.clone()], ..Default::default() }), ..Default::default() })
}
