//! ↩️ Inverse of `SetWallAxis`: an absolute `SetWallAxis` back to the base axis, none when the wall is absent.

use crate::standards::v1::subsets::any::schema::mutations::set_wall_axis::SetWallAxis;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &SetWallAxis, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_wall_axis::SetWallAxis>,
{
    match base.walls.get(&payload.id) {
        Some(wall) => vec![Op::from(SetWallAxis { id: payload.id.clone(), axis: wall.axis.clone() })],
        None => Vec::new(),
    }
}
