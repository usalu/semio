//! ↩️ Inverse (undo) construction for the `surveys` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateSurvey, DeleteSurvey, RenameSurvey, ReplaceSurvey};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateSurvey, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteSurvey(DeleteSurvey { id: payload.survey.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteSurvey, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.surveys.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateSurvey(CreateSurvey { survey: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameSurvey, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.surveys.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameSurvey(RenameSurvey { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceSurvey, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.surveys.iter().find(|row| row.header.id == payload.survey.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceSurvey(ReplaceSurvey { survey: existing.clone() })],
        None => Vec::new(),
    }
}
