//! ↩️ Inverse (undo) construction for the `create-schedule-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📅schedules` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub async fn inverse(payload: &super::mutation::CreateScheduleRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteScheduleRequirement(super::super::delete_schedule_requirement::mutation::DeleteScheduleRequirement { id: payload.schedule_requirement.header.id.clone() })]
}
