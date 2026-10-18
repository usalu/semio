//! ↩️ Inverse of `FlipWall`: the concrete `FlipWall` of the same wall (flipping twice restores the axis exactly), none when the wall is
//! absent or has no direction to flip.

use crate::standards::v1::subsets::any::schema::mutations::wall_geometry::flipped;
use crate::standards::v1::subsets::any::schema::mutations::flip_wall::FlipWall;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &FlipWall, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::flip_wall::FlipWall>,
{
    match base.walls.get(&payload.id) {
        Some(wall) if flipped(&wall.axis) != wall.axis => vec![Op::from(FlipWall { id: payload.id.clone() })],
        _ => Vec::new(),
    }
}
