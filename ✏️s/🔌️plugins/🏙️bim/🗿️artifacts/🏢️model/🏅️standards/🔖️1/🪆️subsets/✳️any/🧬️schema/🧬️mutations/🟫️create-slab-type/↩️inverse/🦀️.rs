//! ↩️ Inverse of `CreateSlabType`: the concrete `DeleteSlabType` of the id it created, none when the id was already taken.

use super::super::delete_slab_type::DeleteSlabType;
use super::CreateSlabType;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateSlabType, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.slab_types.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteSlabType(DeleteSlabType { id: payload.id.clone() })]
}
