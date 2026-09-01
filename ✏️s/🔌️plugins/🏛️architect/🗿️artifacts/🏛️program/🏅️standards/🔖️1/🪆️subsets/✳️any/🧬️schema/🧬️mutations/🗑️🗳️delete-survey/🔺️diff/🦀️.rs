//! 🔺️ Sparse diff construction for the `delete-survey` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🗳️surveys` per Wave C.

use super::DeleteSurvey;
use crate::artifacts::program::diff::ProgramSurveysDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub async fn diff(payload: &DeleteSurvey, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.surveys.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No survey exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { surveys: Some(ProgramSurveysDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
