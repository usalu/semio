//! 🔺️ Sparse diff construction for the `delete-schedule-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📅schedules` per Wave C.

use super::mutation::DeleteScheduleRequirement;
use crate::artifacts::program::diff::ProgramSchedulesDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub async fn diff(payload: &DeleteScheduleRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.schedules.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No schedule requirement exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { schedules: Some(ProgramSchedulesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
