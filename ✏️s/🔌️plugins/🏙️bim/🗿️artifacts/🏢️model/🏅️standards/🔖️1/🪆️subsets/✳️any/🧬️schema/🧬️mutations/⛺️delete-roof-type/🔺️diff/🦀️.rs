//! 🔺️ Diff constructor for `DeleteRoofType`: one deleted roof type entry together with the properties and classifications keyed by the type; refused while a roof still uses the type.

use crate::standards::v1::subsets::any::schema::mutations::delete_roof_type::DeleteRoofType;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::{Entry, KeyedDelta};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteRoofType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.roof_types.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Roof type \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if base.roofs.values().any(|row| row.roof_type == payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Roof type \"{}\" is still used by roofs.", payload.id), [payload.id.clone()]);
    }
    let mut removal = semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::data_diff(base, &payload.id);
    removal.roof_types = Some(KeyedDelta::one(payload.id.clone(), Entry::Deleted));
    MutationOutcome::new(removal)
}
