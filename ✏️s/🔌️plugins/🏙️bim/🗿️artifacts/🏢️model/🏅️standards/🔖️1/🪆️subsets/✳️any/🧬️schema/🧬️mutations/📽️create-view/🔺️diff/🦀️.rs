//! 🔺️ Diff constructor for `CreateView`: one created view entry. The id must be free across every collection, then the view must be writable: its building exists, its
//! name is filled and unique within the building, each kind owns exactly its fields (see `view_problem`), the numbers are positive and finite and the hidden categories are
//! unique and in category order.

use super::super::elements;
use super::CreateView;
use crate::{view_problem, Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateView, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(problem) = view_problem(base, &payload.id, &payload.view) {
        let code = if problem.missing { OutcomeCode::TargetMissing } else { OutcomeCode::Invariant };
        return MutationOutcome::refuse(code, problem.message, ["view", problem.field]);
    }
    MutationOutcome::new(ModelDiff::views(payload.id.clone(), Entry::Created(payload.view.clone())))
}
