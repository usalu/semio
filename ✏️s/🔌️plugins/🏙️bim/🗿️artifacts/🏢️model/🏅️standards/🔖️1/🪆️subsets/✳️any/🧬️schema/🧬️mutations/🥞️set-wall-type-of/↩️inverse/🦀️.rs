//! ↩️ Inverse of `SetWallTypeOf`: an absolute `SetWallTypeOf` back to the base wall type, none when the wall is absent.

use super::SetWallTypeOf;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &SetWallTypeOf, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.walls.get(&payload.id) {
        Some(wall) => vec![ModelMutation::SetWallTypeOf(SetWallTypeOf { id: payload.id.clone(), wall_type: wall.wall_type.clone() })],
        None => Vec::new(),
    }
}
