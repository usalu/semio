//! ↩️ Inverse of `SetWallBaseOffset`: an absolute `SetWallBaseOffset` back to the base offset, none when the wall is absent.

use crate::standards::v1::subsets::any::schema::mutations::set_wall_base_offset::SetWallBaseOffset;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &SetWallBaseOffset, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_wall_base_offset::SetWallBaseOffset>,
{
    match base.walls.get(&payload.id) {
        Some(wall) => vec![Op::from(SetWallBaseOffset { id: payload.id.clone(), base_offset: wall.base_offset })],
        None => Vec::new(),
    }
}
