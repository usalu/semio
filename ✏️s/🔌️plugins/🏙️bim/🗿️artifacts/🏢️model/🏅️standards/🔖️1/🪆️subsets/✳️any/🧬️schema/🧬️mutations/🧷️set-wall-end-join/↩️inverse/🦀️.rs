//! ↩️ Inverse of `SetWallEndJoin`: an absolute `SetWallEndJoin` back to the base preference of that end (absent when the end joined
//! automatically), none when the wall is absent.

use crate::standards::v1::subsets::any::schema::mutations::modify::WallEnd;
use crate::standards::v1::subsets::any::schema::mutations::set_wall_end_join::SetWallEndJoin;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &SetWallEndJoin, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_wall_end_join::SetWallEndJoin>,
{
    match base.walls.get(&payload.id) {
        Some(wall) => vec![Op::from(SetWallEndJoin { id: payload.id.clone(), end: payload.end, join: if payload.end == WallEnd::Start { wall.start_join } else { wall.end_join } })],
        None => Vec::new(),
    }
}
