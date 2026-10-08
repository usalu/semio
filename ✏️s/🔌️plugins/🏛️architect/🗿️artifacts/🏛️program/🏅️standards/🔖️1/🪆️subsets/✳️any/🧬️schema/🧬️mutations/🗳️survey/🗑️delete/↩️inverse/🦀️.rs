//! ↩️ Inverse (undo) construction for the `delete-survey` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🗳️surveys` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteSurvey, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.surveys.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateSurvey(super::super::create_survey::CreateSurvey { survey: base.surveys[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
