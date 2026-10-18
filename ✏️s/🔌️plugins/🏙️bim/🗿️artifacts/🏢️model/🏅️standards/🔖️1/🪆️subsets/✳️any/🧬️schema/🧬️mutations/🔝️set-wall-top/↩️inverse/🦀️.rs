//! ↩️ Inverse of `SetWallTop`: an absolute `SetWallTop` back to the base constraint, none when the wall is absent.

use crate::standards::v1::subsets::any::schema::mutations::set_wall_top::SetWallTop;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &SetWallTop, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_wall_top::SetWallTop>,
{
    match base.walls.get(&payload.id) {
        Some(wall) => vec![Op::from(SetWallTop { id: payload.id.clone(), top: wall.top.clone() })],
        None => Vec::new(),
    }
}
