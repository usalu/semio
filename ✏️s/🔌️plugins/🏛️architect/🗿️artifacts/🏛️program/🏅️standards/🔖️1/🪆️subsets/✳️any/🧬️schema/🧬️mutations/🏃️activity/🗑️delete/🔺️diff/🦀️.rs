//! 🔺️ Sparse diff construction for the `delete-activity` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏃activities` per Wave C.

use super::DeleteActivity;
use crate::artifacts::program::diff::ProgramActivitiesDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub async fn diff(payload: &DeleteActivity, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.activities.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No activity exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { activities: Some(ProgramActivitiesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
