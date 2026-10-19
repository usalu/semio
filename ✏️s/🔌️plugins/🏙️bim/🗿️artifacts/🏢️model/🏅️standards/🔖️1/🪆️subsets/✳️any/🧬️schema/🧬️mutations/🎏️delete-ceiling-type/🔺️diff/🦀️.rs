//! 🔺️ Diff constructor for `DeleteCeilingType`: one deleted ceiling type entry together with the properties and classifications keyed by the type; refused while a ceiling still uses the type.

use crate::standards::v1::subsets::any::schema::mutations::delete_ceiling_type::DeleteCeilingType;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::{Entry, KeyedDelta};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteCeilingType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.ceiling_types.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Ceiling type \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if base.ceilings.values().any(|row| row.ceiling_type == payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Ceiling type \"{}\" is still used by ceilings.", payload.id), [payload.id.clone()]);
    }
    let mut removal = semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::data_diff(base, &payload.id);
    removal.ceiling_types = Some(KeyedDelta::one(payload.id.clone(), Entry::Deleted));
    MutationOutcome::new(removal)
}
