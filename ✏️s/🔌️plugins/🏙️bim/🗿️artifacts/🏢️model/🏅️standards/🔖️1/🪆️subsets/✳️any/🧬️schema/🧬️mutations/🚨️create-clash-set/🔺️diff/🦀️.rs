//! 🔺️ Diff constructor for `CreateClashSet`: one created clash set entry. The id must be free across every collection, then the clash set must be writable (see `clash_set_problem`).

use super::super::elements;
use super::CreateClashSet;
use crate::{clash_set_problem, Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateClashSet, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(problem) = clash_set_problem(base, &payload.id, &payload.clash_set) {
        let code = if problem.missing { OutcomeCode::TargetMissing } else { OutcomeCode::Invariant };
        return MutationOutcome::refuse(code, problem.message, ["clash_set", problem.field]);
    }
    MutationOutcome::new(ModelDiff::clash_sets(payload.id.clone(), Entry::Created(payload.clash_set.clone())))
}
