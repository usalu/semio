//! 🔺️ Sparse diff construction for the `create-schedule-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📅schedules` per Wave C.

use super::CreateScheduleRequirement;
use crate::diff::ProgramSchedulesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateScheduleRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.schedule_requirement.header.id;
    if base.schedules.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A schedule requirement already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.schedules.len());
    if at > base.schedules.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the schedule requirement list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { schedules: Some(ProgramSchedulesDelta::insertion(at, payload.schedule_requirement.clone())), ..Default::default() })
}
