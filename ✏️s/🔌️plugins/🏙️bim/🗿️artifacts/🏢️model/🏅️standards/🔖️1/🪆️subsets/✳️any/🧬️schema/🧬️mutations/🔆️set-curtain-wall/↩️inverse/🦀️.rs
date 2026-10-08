//! ↩️ Inverse of `SetCurtainWall`: an absolute `SetCurtainWall` restoring the base value of exactly the fields the forward really changes, none when the curtain wall is absent or nothing changes.

use super::SetCurtainWall;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetCurtainWall, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.curtain_walls.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetCurtainWall(SetCurtainWall::from_patch(payload.id.clone(), restore))]
}
