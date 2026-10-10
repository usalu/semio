//! 🔺️ Diff constructor for `SetRule`: a sparse rule patch of exactly the provided fields that differ. The rule that results must be writable (see `rule_problem`); a patch that restates the current values is
//! `mutation.no-op`.

use super::SetRule;
use crate::{rule_problem, Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetRule, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.rules.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Rule \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let patch = payload.patch().minimal(record);
    if patch.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Rule \"{}\" already holds these values.", payload.id), [payload.id.clone()]);
    }
    if let Some(problem) = rule_problem(base, &payload.id, &patch.write(record)) {
        let code = if problem.missing { OutcomeCode::TargetMissing } else { OutcomeCode::Invariant };
        return MutationOutcome::refuse(code, problem.message, [problem.field]);
    }
    MutationOutcome::new(ModelDiff::rules(payload.id.clone(), Entry::Patched(patch)))
}
