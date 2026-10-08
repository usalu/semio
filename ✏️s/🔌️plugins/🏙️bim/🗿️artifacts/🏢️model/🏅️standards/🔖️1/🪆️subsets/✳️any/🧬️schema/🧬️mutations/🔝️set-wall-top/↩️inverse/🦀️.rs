//! ↩️ Inverse of `SetWallTop`: an absolute `SetWallTop` back to the base constraint, none when the wall is absent.

use super::SetWallTop;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &SetWallTop, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.walls.get(&payload.id) {
        Some(wall) => vec![ModelMutation::SetWallTop(SetWallTop { id: payload.id.clone(), top: wall.top.clone() })],
        None => Vec::new(),
    }
}
