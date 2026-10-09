//! ↩️ Inverse of `DeleteCeilingType`: the setters of the properties and classifications of the type, then the concrete `CreateCeilingType` carrying the full removed record, none when the ceiling type was absent.

use super::super::create_ceiling_type::CreateCeilingType;
use super::super::cascade;
use super::DeleteCeilingType;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteCeilingType, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.ceiling_types.get(&payload.id) {
        Some(ceiling_type) => cascade::data_rows(base, &payload.id).into_iter().chain(std::iter::once(ModelMutation::CreateCeilingType(CreateCeilingType { id: payload.id.clone(), ceiling_type: ceiling_type.clone() }))).collect(),
        None => Vec::new(),
    }
}
