//! 🔺️ Diff constructor for `RenameStorey`: a one-field storey patch; renaming to the current name is a `mutation.no-op` warning.

use crate::standards::v1::subsets::any::schema::mutations::rename_storey::RenameStorey;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::site_and_spaces::StoreyPatch;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &RenameStorey, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(storey) = base.storeys.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Storey \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if storey.name == payload.name {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Storey \"{}\" already has the name \"{}\".", payload.id, payload.name), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::storeys(payload.id.clone(), Entry::Patched(StoreyPatch { name: Some(payload.name.clone()), ..Default::default() })))
}
