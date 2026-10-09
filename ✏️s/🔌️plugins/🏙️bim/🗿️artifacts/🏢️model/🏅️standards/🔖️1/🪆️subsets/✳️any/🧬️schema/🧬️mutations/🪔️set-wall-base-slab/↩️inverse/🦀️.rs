//! ↩️ Inverse of `SetWallBaseSlab`: an absolute `SetWallBaseSlab` back to the base slab of the wall (none when it stood on its storey), none when the wall is absent.

use super::SetWallBaseSlab;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &SetWallBaseSlab, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.walls.get(&payload.id) {
        Some(wall) => vec![ModelMutation::SetWallBaseSlab(SetWallBaseSlab { id: payload.id.clone(), slab: wall.base_slab.clone() })],
        None => Vec::new(),
    }
}
