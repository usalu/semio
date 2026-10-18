//! ↩️ Inverse of `OffsetWall`: a `DeleteWall` of the created wall, none when the original is absent.

use crate::standards::v1::subsets::any::schema::mutations::delete_wall::DeleteWall;
use crate::standards::v1::subsets::any::schema::mutations::offset_wall::OffsetWall;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &OffsetWall, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_wall::DeleteWall>,
{
    match base.walls.contains_key(&payload.id) {
        true => vec![Op::from(DeleteWall { id: payload.new_id.clone() })],
        false => Vec::new(),
    }
}
