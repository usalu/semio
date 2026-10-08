//! ↩️ Inverse of `DeleteSlabType`: the concrete `CreateSlabType` carrying the full removed record, none when the slab type was absent.

use super::super::create_slab_type::CreateSlabType;
use super::DeleteSlabType;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteSlabType, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.slab_types.get(&payload.id) {
        Some(slab_type) => vec![ModelMutation::CreateSlabType(CreateSlabType { id: payload.id.clone(), slab_type: slab_type.clone() })],
        None => Vec::new(),
    }
}
