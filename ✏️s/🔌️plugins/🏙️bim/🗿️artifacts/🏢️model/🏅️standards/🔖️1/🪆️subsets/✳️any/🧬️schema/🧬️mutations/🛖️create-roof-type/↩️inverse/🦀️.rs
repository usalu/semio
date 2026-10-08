//! ↩️ Inverse of `CreateRoofType`: the concrete `DeleteRoofType` of the id it created, none when the id was already taken.

use super::super::delete_roof_type::DeleteRoofType;
use super::CreateRoofType;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateRoofType, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.roof_types.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteRoofType(DeleteRoofType { id: payload.id.clone() })]
}
