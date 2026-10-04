//! ↩️ Inverse (undo) construction for the `create-schedule-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📅schedules` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::CreateScheduleRequirement, _base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![ProgramMutation::DeleteScheduleRequirement(super::super::delete_schedule_requirement::DeleteScheduleRequirement { id: payload.schedule_requirement.header.id.clone() })]

    })())
}
