//! ↩️ Inverse of `CreateDoorType`: the concrete `DeleteDoorType` of the id it created, none when the id was already taken.

use super::super::delete_door_type::DeleteDoorType;
use super::CreateDoorType;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateDoorType, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.door_types.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteDoorType(DeleteDoorType { id: payload.id.clone() })]
}
