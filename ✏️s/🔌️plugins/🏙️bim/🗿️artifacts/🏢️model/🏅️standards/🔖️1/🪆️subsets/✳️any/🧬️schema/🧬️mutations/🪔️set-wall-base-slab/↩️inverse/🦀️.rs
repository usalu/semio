//! ↩️ Inverse of `SetWallBaseSlab`: an absolute `SetWallBaseSlab` back to the base slab of the wall (none when it stood on its storey), none when the wall is absent.

use crate::standards::v1::subsets::any::schema::mutations::set_wall_base_slab::SetWallBaseSlab;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &SetWallBaseSlab, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_wall_base_slab::SetWallBaseSlab>,
{
    match base.walls.get(&payload.id) {
        Some(wall) => vec![Op::from(SetWallBaseSlab { id: payload.id.clone(), slab: wall.base_slab.clone() })],
        None => Vec::new(),
    }
}
