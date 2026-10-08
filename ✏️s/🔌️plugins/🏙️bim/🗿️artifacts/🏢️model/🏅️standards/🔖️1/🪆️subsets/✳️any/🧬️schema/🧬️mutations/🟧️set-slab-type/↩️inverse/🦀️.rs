//! ↩️ Inverse of `SetSlabType`: an absolute `SetSlabType` restoring the base value of exactly the fields the forward really changes, none when the slab type is absent or nothing changes.

use super::SetSlabType;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetSlabType, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.slab_types.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetSlabType(SetSlabType::from_patch(payload.id.clone(), restore))]
}
