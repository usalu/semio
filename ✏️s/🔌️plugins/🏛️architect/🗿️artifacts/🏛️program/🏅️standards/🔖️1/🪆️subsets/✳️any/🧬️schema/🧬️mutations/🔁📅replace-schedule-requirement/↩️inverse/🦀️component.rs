//! ↩️ Inverse (undo) construction for the `replace-schedule-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📅schedules` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::ReplaceScheduleRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.schedules.iter().find(|row| row.header.id == payload.schedule_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceScheduleRequirement(super::mutation::ReplaceScheduleRequirement { schedule_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
