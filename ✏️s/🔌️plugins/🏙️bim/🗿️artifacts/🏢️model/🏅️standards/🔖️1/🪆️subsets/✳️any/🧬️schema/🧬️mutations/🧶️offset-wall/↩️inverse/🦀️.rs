//! ↩️ Inverse of `OffsetWall`: a `DeleteWall` of the created wall, none when the original is absent.

use super::super::delete_wall::DeleteWall;
use super::OffsetWall;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &OffsetWall, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.walls.contains_key(&payload.id) {
        true => vec![ModelMutation::DeleteWall(DeleteWall { id: payload.new_id.clone() })],
        false => Vec::new(),
    }
}
