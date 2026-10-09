//! 🔺️ Diff constructor for `CreateViewport`: one created viewport entry. The id must be free across every collection, then the viewport must be writable (see `viewport_problem`).

use super::super::elements;
use super::CreateViewport;
use crate::{viewport_problem, Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateViewport, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(problem) = viewport_problem(base, &payload.id, &payload.viewport) {
        let code = if problem.missing { OutcomeCode::TargetMissing } else { OutcomeCode::Invariant };
        return MutationOutcome::refuse(code, problem.message, ["viewport", problem.field]);
    }
    MutationOutcome::new(ModelDiff::viewports(payload.id.clone(), Entry::Created(payload.viewport.clone())))
}
