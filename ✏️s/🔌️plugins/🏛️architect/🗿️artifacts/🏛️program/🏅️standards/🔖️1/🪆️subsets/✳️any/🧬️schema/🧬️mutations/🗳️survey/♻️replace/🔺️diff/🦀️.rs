//! 🔺️ Sparse diff construction for the `replace-survey` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🗳️surveys` per Wave C.

use super::ReplaceSurvey;
use crate::diff::ProgramSurveysDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns:
/// `removed = [id]`, `added = [payload row]`, and `reordered` (the base order) unless the row was last, so the new row keeps its position.
pub fn diff(payload: &ReplaceSurvey, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.survey.header.id;
    let Some(position) = base.surveys.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No survey exists with this id.", [id.0.clone()]);
    };
    if base.surveys[position] == payload.survey {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This survey already matches the requested value.").at([id.0.clone()])]);
    }
    let reordered = (position + 1 != base.surveys.len()).then(|| base.surveys.iter().map(|row| row.header.id.0.clone()).collect());
    protocol::MutationOutcome::new(ProgramDiff { surveys: Some(ProgramSurveysDelta { removed: vec![id.0.clone()], added: vec![payload.survey.clone()], reordered, ..Default::default() }), ..Default::default() })
}
