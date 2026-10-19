//! ↩️ Inverse of `OffsetWall`: a `DeleteWall` of the created wall, none when the original is absent.

use semio_s_artifact_bim_model_mut_any_61::standards::v1::subsets::any::schema::mutations::delete_wall::DeleteWall;
use crate::standards::v1::subsets::any::schema::mutations::offset_wall::OffsetWall;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &OffsetWall, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<semio_s_artifact_bim_model_mut_any_61::standards::v1::subsets::any::schema::mutations::delete_wall::DeleteWall>,
{
    match base.walls.contains_key(&payload.id) {
        true => vec![Op::from(DeleteWall { id: payload.new_id.clone() })],
        false => Vec::new(),
    }
}
