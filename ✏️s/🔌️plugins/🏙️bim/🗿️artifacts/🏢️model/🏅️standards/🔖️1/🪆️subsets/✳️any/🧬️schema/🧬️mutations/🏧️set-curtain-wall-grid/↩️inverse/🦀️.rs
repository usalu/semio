//! ↩️ Inverse of `SetCurtainWallGrid`: an absolute `SetCurtainWallGrid` restoring the base value of exactly the fields the forward really changes, none when the curtain wall is absent or nothing changes.

use super::SetCurtainWallGrid;
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot, diff::Patch};

pub fn inverse(payload: &SetCurtainWallGrid, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.curtain_walls.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetCurtainWallGrid(SetCurtainWallGrid::from_patch(payload.id.clone(), restore))]
}
