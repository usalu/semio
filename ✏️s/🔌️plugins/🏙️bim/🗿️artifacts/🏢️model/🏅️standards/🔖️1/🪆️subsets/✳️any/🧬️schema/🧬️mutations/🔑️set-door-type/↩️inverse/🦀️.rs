//! ↩️ Inverse of `SetDoorType`: an absolute `SetDoorType` restoring the base value of exactly the fields the forward really changes, none when the door type is absent or nothing changes.

use super::SetDoorType;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetDoorType, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.door_types.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).negate(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetDoorType(SetDoorType::from_patch(payload.id.clone(), restore))]
}
