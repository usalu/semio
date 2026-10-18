//! 🔺️ Diff constructor for `DeleteBeamType`: one deleted beam type entry together with the properties and classifications keyed by the type; refused while beams still use it.

use crate::standards::v1::subsets::any::schema::mutations::cascade;
use crate::standards::v1::subsets::any::schema::mutations::delete_beam_type::DeleteBeamType;
use crate::standards::v1::subsets::any::schema::{diff::{Entry, KeyedDelta, ModelDiff}, snapshot::ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteBeamType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.beam_types.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Beam type \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if base.beams.values().any(|row| row.beam_type == payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Beam type \"{}\" is still used by beams.", payload.id), [payload.id.clone()]);
    }
    let mut removal = cascade::data_diff(base, &payload.id);
    removal.beam_types = Some(KeyedDelta::one(payload.id.clone(), Entry::Deleted));
    MutationOutcome::new(removal)
}
