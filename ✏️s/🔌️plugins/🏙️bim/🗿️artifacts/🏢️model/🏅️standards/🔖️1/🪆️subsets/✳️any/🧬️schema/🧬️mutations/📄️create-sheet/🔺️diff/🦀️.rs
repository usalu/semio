//! 🔺️ Diff constructor for `CreateSheet`: one created sheet entry. The id must be free across every collection, then the sheet must be writable (see `sheet_problem`).

use super::super::elements;
use super::CreateSheet;
use crate::{sheet_problem, Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateSheet, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(problem) = sheet_problem(base, &payload.id, &payload.sheet) {
        let code = if problem.missing { OutcomeCode::TargetMissing } else { OutcomeCode::Invariant };
        return MutationOutcome::refuse(code, problem.message, ["sheet", problem.field]);
    }
    MutationOutcome::new(ModelDiff::sheets(payload.id.clone(), Entry::Created(payload.sheet.clone())))
}
