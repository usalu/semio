//! 🔺️ Diff constructor for `SetClashSet`: a sparse clash set patch of exactly the provided fields that differ. The clash set that results must be writable (see `clash_set_problem`); a patch that restates the current values is
//! `mutation.no-op`.

use super::SetClashSet;
use crate::{clash_set_problem, Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetClashSet, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.clash_sets.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Clash set \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let patch = payload.patch().minimal(record);
    if patch.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Clash set \"{}\" already holds these values.", payload.id), [payload.id.clone()]);
    }
    if let Some(problem) = clash_set_problem(base, &payload.id, &patch.write(record)) {
        let code = if problem.missing { OutcomeCode::TargetMissing } else { OutcomeCode::Invariant };
        return MutationOutcome::refuse(code, problem.message, [problem.field]);
    }
    MutationOutcome::new(ModelDiff::clash_sets(payload.id.clone(), Entry::Patched(patch)))
}
