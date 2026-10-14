//! ↩️ Inverse of `SetWallLocation`: an absolute `SetWallLocation` back to the base location line, none when the wall is absent.

use super::SetWallLocation;
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot};

pub fn inverse(payload: &SetWallLocation, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.walls.get(&payload.id) {
        Some(wall) => vec![ModelMutation::SetWallLocation(SetWallLocation { id: payload.id.clone(), location: wall.location })],
        None => Vec::new(),
    }
}
