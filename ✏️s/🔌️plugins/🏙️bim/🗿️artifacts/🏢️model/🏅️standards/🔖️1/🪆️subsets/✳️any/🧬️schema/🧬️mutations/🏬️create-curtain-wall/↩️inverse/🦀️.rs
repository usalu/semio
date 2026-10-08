//! ↩️ Inverse of `CreateCurtainWall`: the concrete `DeleteCurtainWall` of the id it created, none when the id was already taken.

use super::super::delete_curtain_wall::DeleteCurtainWall;
use super::CreateCurtainWall;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateCurtainWall, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.curtain_walls.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteCurtainWall(DeleteCurtainWall { id: payload.id.clone() })]
}
