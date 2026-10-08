//! 🔺️ Sparse diff construction for the `create-survey` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🗳️surveys` per Wave C.

use super::CreateSurvey;
use crate::diff::ProgramSurveysDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateSurvey, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.survey.header.id;
    if base.surveys.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A survey already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.surveys.len());
    if at > base.surveys.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the survey list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { surveys: Some(ProgramSurveysDelta::insertion(at, payload.survey.clone())), ..Default::default() })
}
