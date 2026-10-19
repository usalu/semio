//! 🔺️ Diff constructor for `SetStoreyLevel`: a one-field storey patch; the level index stays unique within the building.

use crate::standards::v1::subsets::any::schema::mutations::set_storey_level::SetStoreyLevel;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::site_and_spaces::StoreyPatch;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetStoreyLevel, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(storey) = base.storeys.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Storey \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if storey.level == payload.level {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Storey \"{}\" already is on level {}.", payload.id, payload.level), [payload.id.clone()]);
    }
    if base.storeys.iter().any(|(other, row)| *other != payload.id && row.building == storey.building && row.level == payload.level) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, format!("Level {} is already taken in building \"{}\".", payload.level, storey.building), ["level"]);
    }
    MutationOutcome::new(ModelDiff::storeys(payload.id.clone(), Entry::Patched(StoreyPatch { level: Some(payload.level), ..Default::default() })))
}
