//! ↩️ Inverse (undo) construction for the `delete-schedule-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📅schedules` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::DeleteScheduleRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.schedules.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateScheduleRequirement(super::super::create_schedule_requirement::mutation::CreateScheduleRequirement { schedule_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
