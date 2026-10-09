//! ↩️ Inverse of `CreateCurtainWallType`: the concrete `DeleteCurtainWallType` of the id it created, none when the id was already taken.

use super::super::delete_curtain_wall_type::DeleteCurtainWallType;
use super::CreateCurtainWallType;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateCurtainWallType, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.curtain_wall_types.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteCurtainWallType(DeleteCurtainWallType { id: payload.id.clone() })]
}
