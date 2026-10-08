//! ↩️ Inverse of `FlipWall`: the concrete `FlipWall` of the same wall (flipping twice restores the axis exactly), none when the wall is
//! absent or has no direction to flip.

use super::super::wall_geometry::flipped;
use super::FlipWall;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &FlipWall, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.walls.get(&payload.id) {
        Some(wall) if flipped(&wall.axis) != wall.axis => vec![ModelMutation::FlipWall(FlipWall { id: payload.id.clone() })],
        _ => Vec::new(),
    }
}
