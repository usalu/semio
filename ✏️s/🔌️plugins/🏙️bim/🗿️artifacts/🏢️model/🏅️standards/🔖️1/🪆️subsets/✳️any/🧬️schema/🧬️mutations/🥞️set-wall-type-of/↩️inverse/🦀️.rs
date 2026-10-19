//! ↩️ Inverse of `SetWallTypeOf`: an absolute `SetWallTypeOf` back to the base wall type, none when the wall is absent.

use crate::standards::v1::subsets::any::schema::mutations::set_wall_type_of::SetWallTypeOf;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &SetWallTypeOf, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_wall_type_of::SetWallTypeOf>,
{
    match base.walls.get(&payload.id) {
        Some(wall) => vec![Op::from(SetWallTypeOf { id: payload.id.clone(), wall_type: wall.wall_type.clone() })],
        None => Vec::new(),
    }
}
