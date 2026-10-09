//! ↩️ Inverse of `SetCeilingType`: an absolute `SetCeilingType` restoring the base value of exactly the fields the forward really changes, none when the ceiling type is absent or nothing changes.

use super::SetCeilingType;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetCeilingType, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.ceiling_types.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetCeilingType(SetCeilingType::from_patch(payload.id.clone(), restore))]
}
