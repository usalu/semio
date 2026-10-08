//! ↩️ Inverse (undo) construction for the `delete-activity` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🏃activities` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteActivity, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.activities.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateActivity(super::super::create_activity::CreateActivity { activity: base.activities[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
