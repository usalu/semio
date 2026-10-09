//! 🔺️ Diff constructor for `DeleteRoofType`: one deleted roof type entry together with the properties and classifications keyed by the type; refused while a roof still uses the type.

use super::super::cascade;
use super::DeleteRoofType;
use crate::{Entry, KeyedDelta, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteRoofType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.roof_types.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Roof type \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if base.roofs.values().any(|row| row.roof_type == payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Roof type \"{}\" is still used by roofs.", payload.id), [payload.id.clone()]);
    }
    let mut removal = cascade::data_diff(base, &payload.id);
    removal.roof_types = Some(KeyedDelta::one(payload.id.clone(), Entry::Deleted));
    MutationOutcome::new(removal)
}
