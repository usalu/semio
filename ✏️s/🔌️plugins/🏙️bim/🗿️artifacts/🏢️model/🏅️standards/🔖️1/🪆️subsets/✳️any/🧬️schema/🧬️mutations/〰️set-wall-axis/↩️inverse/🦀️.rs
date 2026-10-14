//! ↩️ Inverse of `SetWallAxis`: an absolute `SetWallAxis` back to the base axis, none when the wall is absent.

use super::SetWallAxis;
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot};

pub fn inverse(payload: &SetWallAxis, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.walls.get(&payload.id) {
        Some(wall) => vec![ModelMutation::SetWallAxis(SetWallAxis { id: payload.id.clone(), axis: wall.axis.clone() })],
        None => Vec::new(),
    }
}
