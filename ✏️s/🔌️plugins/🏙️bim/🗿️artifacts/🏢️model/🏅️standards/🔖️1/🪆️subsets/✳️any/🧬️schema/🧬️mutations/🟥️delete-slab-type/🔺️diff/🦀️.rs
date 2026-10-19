//! 🔺️ Diff constructor for `DeleteSlabType`: one deleted slab type entry together with the properties and classifications keyed by the type; refused while a slab still uses the type.

use crate::standards::v1::subsets::any::schema::mutations::delete_slab_type::DeleteSlabType;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::{Entry, KeyedDelta};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteSlabType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.slab_types.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Slab type \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if base.slabs.values().any(|row| row.slab_type == payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Slab type \"{}\" is still used by slabs.", payload.id), [payload.id.clone()]);
    }
    let mut removal = semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::data_diff(base, &payload.id);
    removal.slab_types = Some(KeyedDelta::one(payload.id.clone(), Entry::Deleted));
    MutationOutcome::new(removal)
}
