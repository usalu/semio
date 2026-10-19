//! 🔺️ Sparse declarative delete-load-case diff; no derived state is evaluated.
use crate::standards::v1::subsets::any::schema::mutations::delete_load_case::DeleteLoadCase;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};
pub fn diff(payload: &DeleteLoadCase, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {

if !base.load_cases.contains_key(&payload.id) { return MutationOutcome::refuse(OutcomeCode::TargetMissing, "Structural record does not exist.", [payload.id.clone()]); }
if base.loads.values().any(|row| row.load_case == payload.id) { return MutationOutcome::refuse(OutcomeCode::TargetReferenced, "Load case is referenced by loads.", [payload.id.clone()]); }
MutationOutcome::new(ModelDiff::load_cases(payload.id.clone(), Entry::Deleted))
}
