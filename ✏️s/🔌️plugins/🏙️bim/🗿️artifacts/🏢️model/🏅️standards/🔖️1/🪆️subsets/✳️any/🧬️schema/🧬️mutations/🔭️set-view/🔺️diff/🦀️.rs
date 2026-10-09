//! 🔺️ Diff constructor for `SetView`: a sparse view patch of exactly the provided fields that differ. The view that results must be writable (see `view_problem`): a kind
//! owns exactly its fields, so a plane on a plan, a camera on a section or a storey on an elevation is refused, a new storey must exist in the building of the view, the name
//! stays unique and the numbers stay positive and finite. A patch that restates the current values is a `mutation.no-op`.

use super::SetView;
use crate::{view_problem, Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetView, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(view) = base.views.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("View \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let patch = payload.patch().minimal(view);
    if patch.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("View \"{}\" already holds these values.", payload.id), [payload.id.clone()]);
    }
    if let Some(problem) = view_problem(base, &payload.id, &patch.write(view)) {
        let code = if problem.missing { OutcomeCode::TargetMissing } else { OutcomeCode::Invariant };
        return MutationOutcome::refuse(code, problem.message, [problem.field]);
    }
    MutationOutcome::new(ModelDiff::views(payload.id.clone(), Entry::Patched(patch)))
}
