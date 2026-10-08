//! ↩️ Inverse of `CreateWall`: the concrete `DeleteWall` of the id it created, none when the id was already taken.

use super::super::delete_wall::DeleteWall;
use super::CreateWall;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateWall, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.walls.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteWall(DeleteWall { id: payload.id.clone() })]
}
