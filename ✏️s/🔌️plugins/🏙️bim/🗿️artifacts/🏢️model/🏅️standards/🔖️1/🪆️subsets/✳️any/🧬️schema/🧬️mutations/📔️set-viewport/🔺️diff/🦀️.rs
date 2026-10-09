//! 🔺️ Diff constructor for `SetViewport`: a sparse viewport patch of exactly the provided fields that differ. The viewport that results must be writable (see `viewport_problem`); a patch that restates the current values is a
//! `mutation.no-op`.

use super::SetViewport;
use crate::{viewport_problem, Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetViewport, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.viewports.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Viewport \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let patch = payload.patch().minimal(record);
    if patch.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Viewport \"{}\" already holds these values.", payload.id), [payload.id.clone()]);
    }
    if let Some(problem) = viewport_problem(base, &payload.id, &patch.write(record)) {
        let code = if problem.missing { OutcomeCode::TargetMissing } else { OutcomeCode::Invariant };
        return MutationOutcome::refuse(code, problem.message, [problem.field]);
    }
    MutationOutcome::new(ModelDiff::viewports(payload.id.clone(), Entry::Patched(patch)))
}
