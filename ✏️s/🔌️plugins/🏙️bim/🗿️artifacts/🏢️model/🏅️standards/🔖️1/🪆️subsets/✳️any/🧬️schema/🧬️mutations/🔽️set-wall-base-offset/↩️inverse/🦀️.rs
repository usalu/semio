//! ↩️ Inverse of `SetWallBaseOffset`: an absolute `SetWallBaseOffset` back to the base offset, none when the wall is absent.

use super::SetWallBaseOffset;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &SetWallBaseOffset, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.walls.get(&payload.id) {
        Some(wall) => vec![ModelMutation::SetWallBaseOffset(SetWallBaseOffset { id: payload.id.clone(), base_offset: wall.base_offset })],
        None => Vec::new(),
    }
}
