//! ↩️ Inverse of `CreateWallType`: the concrete `DeleteWallType` of the id it created, none when the id was already taken.

use super::super::delete_wall_type::DeleteWallType;
use super::CreateWallType;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateWallType, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.wall_types.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteWallType(DeleteWallType { id: payload.id.clone() })]
}
