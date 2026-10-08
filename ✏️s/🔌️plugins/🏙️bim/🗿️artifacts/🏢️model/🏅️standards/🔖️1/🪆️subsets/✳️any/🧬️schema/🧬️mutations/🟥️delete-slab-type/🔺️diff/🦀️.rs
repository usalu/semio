//! 🔺️ Diff constructor for `DeleteSlabType`: one deleted slab type entry; refused while a slab still uses the type.

use super::DeleteSlabType;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteSlabType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.slab_types.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Slab type \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if base.slabs.values().any(|row| row.slab_type == payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Slab type \"{}\" is still used by slabs.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::slab_types(payload.id.clone(), Entry::Deleted))
}
