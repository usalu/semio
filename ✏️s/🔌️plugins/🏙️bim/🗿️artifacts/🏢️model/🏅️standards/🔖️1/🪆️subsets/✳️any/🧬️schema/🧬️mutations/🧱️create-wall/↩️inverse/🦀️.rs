//! ↩️ Inverse of `CreateWall`: the concrete `DeleteWall` of the id it created, none when the id was already taken.

use semio_s_artifact_bim_model_mut_any_61::standards::v1::subsets::any::schema::mutations::delete_wall::DeleteWall;
use crate::standards::v1::subsets::any::schema::mutations::create_wall::CreateWall;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateWall, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<semio_s_artifact_bim_model_mut_any_61::standards::v1::subsets::any::schema::mutations::delete_wall::DeleteWall>,
{
    if base.walls.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteWall { id: payload.id.clone() })]
}
