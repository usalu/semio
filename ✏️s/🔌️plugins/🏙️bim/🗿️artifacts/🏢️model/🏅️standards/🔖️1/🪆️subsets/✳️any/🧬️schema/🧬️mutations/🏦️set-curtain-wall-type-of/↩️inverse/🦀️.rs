//! ↩️ Inverse of `SetCurtainWallTypeOf`: an absolute `SetCurtainWallTypeOf` back to the base curtain wall type, none when the curtain wall is absent.

use super::SetCurtainWallTypeOf;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &SetCurtainWallTypeOf, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.curtain_walls.get(&payload.id) {
        Some(wall) => vec![ModelMutation::SetCurtainWallTypeOf(SetCurtainWallTypeOf { id: payload.id.clone(), curtain_wall_type: wall.curtain_wall_type.clone() })],
        None => Vec::new(),
    }
}
