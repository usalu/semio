//! 🔺️ Diff constructor for `DeleteSlabType`: one deleted slab type entry together with the properties and classifications keyed by the type; refused while a slab still uses the type.

use super::super::cascade;
use super::DeleteSlabType;
use crate::{Entry, KeyedDelta, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteSlabType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.slab_types.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Slab type \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if base.slabs.values().any(|row| row.slab_type == payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Slab type \"{}\" is still used by slabs.", payload.id), [payload.id.clone()]);
    }
    let mut removal = cascade::data_diff(base, &payload.id);
    removal.slab_types = Some(KeyedDelta::one(payload.id.clone(), Entry::Deleted));
    MutationOutcome::new(removal)
}
