//! ↩️ Inverse of `CreateCeilingType`: the concrete `DeleteCeilingType` of the id it created, none when the id was already taken.

use super::super::delete_ceiling_type::DeleteCeilingType;
use super::CreateCeilingType;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateCeilingType, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.ceiling_types.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteCeilingType(DeleteCeilingType { id: payload.id.clone() })]
}
