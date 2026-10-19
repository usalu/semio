//! 🔺️ Sparse declarative set-load-case diff; no derived state is evaluated.
use crate::standards::v1::subsets::any::schema::mutations::set_load_case::SetLoadCase;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::{Entry, Patch};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};
pub fn diff(payload: &SetLoadCase, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {

let Some(record) = base.load_cases.get(&payload.id) else { return MutationOutcome::refuse(OutcomeCode::TargetMissing, "Structural record does not exist.", [payload.id.clone()]); };
let patch = payload.patch().minimal(record); if patch.is_empty() { return MutationOutcome::refuse(OutcomeCode::NoOp, "Structural parameters are unchanged.", [payload.id.clone()]); }
if let Some(problem) = semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::load_case_problem(base, &patch.write(record)) { return MutationOutcome::refuse(OutcomeCode::Invariant, problem, [payload.id.clone()]); }
MutationOutcome::new(ModelDiff::load_cases(payload.id.clone(), Entry::Patched(patch)))
}
