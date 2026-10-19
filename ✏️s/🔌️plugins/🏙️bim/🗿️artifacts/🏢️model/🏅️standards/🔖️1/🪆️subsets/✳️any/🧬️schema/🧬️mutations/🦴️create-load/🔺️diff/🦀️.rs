//! 🔺️ Sparse declarative create-load diff; no derived state is evaluated.
use crate::standards::v1::subsets::any::schema::mutations::create_load::CreateLoad;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};
pub fn diff(payload: &CreateLoad, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
if let Some(noun) = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::taken(base, &payload.id) { return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} already exists."), [payload.id.clone()]); }
if let Some(problem) = semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::load_problem(base, &payload.load) { return MutationOutcome::refuse(OutcomeCode::Invariant, problem, [payload.id.clone()]); }
MutationOutcome::new(ModelDiff::loads(payload.id.clone(), Entry::Created(payload.load.clone())))
}
