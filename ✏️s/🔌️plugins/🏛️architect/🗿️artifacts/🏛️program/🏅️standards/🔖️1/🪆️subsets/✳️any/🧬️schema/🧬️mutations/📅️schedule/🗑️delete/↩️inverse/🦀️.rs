//! ↩️ Inverse (undo) construction for the `delete-schedule-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📅schedules` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteScheduleRequirement, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.schedules.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateScheduleRequirement(super::super::create_schedule_requirement::CreateScheduleRequirement { schedule_requirement: base.schedules[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
