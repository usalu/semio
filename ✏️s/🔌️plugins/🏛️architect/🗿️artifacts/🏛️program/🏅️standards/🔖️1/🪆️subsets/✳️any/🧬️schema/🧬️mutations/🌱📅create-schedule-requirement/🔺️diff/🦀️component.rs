//! 🔺️ Sparse diff construction for the `create-schedule-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📅schedules` per Wave C.

use super::mutation::CreateScheduleRequirement;
use crate::artifacts::program::diff::ProgramSchedulesDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub async fn diff(payload: &CreateScheduleRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.schedule_requirement.header.id.clone();
    if base.schedules.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A schedule requirement already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { schedules: Some(ProgramSchedulesDelta { added: vec![payload.schedule_requirement.clone()], ..Default::default() }), ..Default::default() })
}
