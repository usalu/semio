//! 🔺️ Diff constructor for `DeleteBeamType`: one deleted beam type entry together with the properties and classifications keyed by the type; refused while beams still use it.

use crate::standards::v1::subsets::any::schema::mutations::delete_beam_type::DeleteBeamType;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::{Entry, KeyedDelta};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteBeamType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.beam_types.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Beam type \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if base.beams.values().any(|row| row.beam_type == payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Beam type \"{}\" is still used by beams.", payload.id), [payload.id.clone()]);
    }
    let mut removal = semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::data_diff(base, &payload.id);
    removal.beam_types = Some(KeyedDelta::one(payload.id.clone(), Entry::Deleted));
    MutationOutcome::new(removal)
}
