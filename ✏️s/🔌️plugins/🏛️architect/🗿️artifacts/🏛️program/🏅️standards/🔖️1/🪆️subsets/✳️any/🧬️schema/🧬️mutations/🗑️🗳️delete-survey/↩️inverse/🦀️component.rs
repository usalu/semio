//! ↩️ Inverse (undo) construction for the `delete-survey` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🗳️surveys` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::DeleteSurvey, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.surveys.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateSurvey(super::super::create_survey::mutation::CreateSurvey { survey: existing.clone() })],
        None => Vec::new(),
    }
}
