//! ↩️ Inverse of `SetWallLocation`: an absolute `SetWallLocation` back to the base location line, none when the wall is absent.

use crate::standards::v1::subsets::any::schema::mutations::set_wall_location::SetWallLocation;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &SetWallLocation, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_wall_location::SetWallLocation>,
{
    match base.walls.get(&payload.id) {
        Some(wall) => vec![Op::from(SetWallLocation { id: payload.id.clone(), location: wall.location })],
        None => Vec::new(),
    }
}
