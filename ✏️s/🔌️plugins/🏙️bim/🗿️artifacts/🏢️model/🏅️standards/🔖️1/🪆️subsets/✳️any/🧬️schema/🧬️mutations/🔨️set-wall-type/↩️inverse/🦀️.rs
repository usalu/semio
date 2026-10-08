//! ↩️ Inverse of `SetWallType`: an absolute `SetWallType` restoring the base value of exactly the fields the forward really changes, none when the wall type is absent or nothing changes.

use super::SetWallType;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetWallType, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.wall_types.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).negate(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetWallType(SetWallType::from_patch(payload.id.clone(), restore))]
}
