//! 🔺️ Sparse declarative set-design-option diff.
use crate::standards::v1::subsets::any::schema::mutations::set_design_option::SetDesignOption;
use crate::standards::v1::subsets::any::schema::{diff::{Entry, ModelDiff, Patch}, snapshot::ModelSnapshot};
use crate::standards::v1::subsets::any::schema::authored::option_rules;
use protocol::{MutationOutcome, OutcomeCode};
pub fn diff(payload: &SetDesignOption, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.design_options.get(&payload.id) else { return MutationOutcome::refuse(OutcomeCode::TargetMissing, "Record is missing.", [payload.id.clone()]); };
    let patch = payload.patch().minimal(record);
    if patch.is_empty() { return MutationOutcome::refuse(OutcomeCode::NoOp, "Values are unchanged.", [payload.id.clone()]); }
    let value = patch.write(record);
    if let Some(problem) = option_rules::option_problem(base, &payload.id, &value) { return MutationOutcome::refuse(OutcomeCode::Invariant, problem, [payload.id.clone()]); }
    MutationOutcome::new(ModelDiff::design_options(payload.id.clone(), Entry::Patched(patch)))
}
