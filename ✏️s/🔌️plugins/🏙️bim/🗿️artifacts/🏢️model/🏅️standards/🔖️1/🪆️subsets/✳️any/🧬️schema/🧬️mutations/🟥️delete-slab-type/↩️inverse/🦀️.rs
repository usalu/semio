//! ↩️ Inverse of `DeleteSlabType`: the setters of the properties and classifications of the type, then the concrete `CreateSlabType` carrying the full removed record, none when the slab type was absent.

use super::super::create_slab_type::CreateSlabType;
use super::super::cascade;
use super::DeleteSlabType;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteSlabType, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.slab_types.get(&payload.id) {
        Some(slab_type) => cascade::data_rows(base, &payload.id).into_iter().chain(std::iter::once(ModelMutation::CreateSlabType(CreateSlabType { id: payload.id.clone(), slab_type: slab_type.clone() }))).collect(),
        None => Vec::new(),
    }
}
