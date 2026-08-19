//! ↩️ Inverse (undo) construction for the `create-survey` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🗳️surveys` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub async fn inverse(payload: &super::mutation::CreateSurvey, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteSurvey(super::super::delete_survey::mutation::DeleteSurvey { id: payload.survey.header.id.clone() })]
}
