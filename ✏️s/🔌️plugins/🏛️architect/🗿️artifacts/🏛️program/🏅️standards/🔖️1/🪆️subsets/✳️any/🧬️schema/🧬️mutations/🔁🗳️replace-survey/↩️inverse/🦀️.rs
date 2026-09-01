//! ↩️ Inverse (undo) construction for the `replace-survey` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🗳️surveys` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::ReplaceSurvey, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.surveys.iter().find(|row| row.header.id == payload.survey.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceSurvey(super::ReplaceSurvey { survey: existing.clone() })],
        None => Vec::new(),
    }
}
