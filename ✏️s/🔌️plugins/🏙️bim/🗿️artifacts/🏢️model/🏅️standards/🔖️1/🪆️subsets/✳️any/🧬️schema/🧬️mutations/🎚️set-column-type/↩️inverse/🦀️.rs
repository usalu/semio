//! ↩️ Inverse of `SetColumnType`: an absolute `SetColumnType` restoring the base value of exactly the fields the forward really changes, none when the column type is absent or nothing changes.

use super::SetColumnType;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetColumnType, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.column_types.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).negate(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetColumnType(SetColumnType::from_patch(payload.id.clone(), restore))]
}
