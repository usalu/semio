//! 🔺️ Sparse declarative set-design-option diff.
use crate::standards::v1::subsets::any::schema::mutations::set_design_option::SetDesignOption;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::{Entry, Patch};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};
pub fn diff(payload: &SetDesignOption, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.design_options.get(&payload.id) else { return MutationOutcome::refuse(OutcomeCode::TargetMissing, "Record is missing.", [payload.id.clone()]); };
    let patch = payload.patch().minimal(record);
    if patch.is_empty() { return MutationOutcome::refuse(OutcomeCode::NoOp, "Values are unchanged.", [payload.id.clone()]); }
    let value = patch.write(record);
    if let Some(problem) = semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::authored::option_rules::option_problem(base, &payload.id, &value) { return MutationOutcome::refuse(OutcomeCode::Invariant, problem, [payload.id.clone()]); }
    MutationOutcome::new(ModelDiff::design_options(payload.id.clone(), Entry::Patched(patch)))
}
