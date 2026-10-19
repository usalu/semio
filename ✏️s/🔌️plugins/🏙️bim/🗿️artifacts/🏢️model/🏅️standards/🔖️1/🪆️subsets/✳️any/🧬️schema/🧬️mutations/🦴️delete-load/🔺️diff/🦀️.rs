//! 🔺️ Sparse declarative delete-load diff; no derived state is evaluated.
use crate::standards::v1::subsets::any::schema::mutations::delete_load::DeleteLoad;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};
pub fn diff(payload: &DeleteLoad, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {

if !base.loads.contains_key(&payload.id) { return MutationOutcome::refuse(OutcomeCode::TargetMissing, "Structural record does not exist.", [payload.id.clone()]); }

MutationOutcome::new(ModelDiff::loads(payload.id.clone(), Entry::Deleted))
}
