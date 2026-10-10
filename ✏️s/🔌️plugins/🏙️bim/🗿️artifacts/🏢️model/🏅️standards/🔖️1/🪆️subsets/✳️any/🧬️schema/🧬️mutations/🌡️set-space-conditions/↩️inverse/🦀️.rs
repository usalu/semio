//! ↩️ Inverse of `SetSpaceConditions`: `RemoveSpaceConditions` when the forward creates the record, else an absolute `SetSpaceConditions` restoring the base value of exactly the fields the forward really changes.

use super::SetSpaceConditions;
use crate::{ModelMutation, ModelSnapshot, Patch};
use super::super::remove_space_conditions::RemoveSpaceConditions;

pub fn inverse(payload: &SetSpaceConditions, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if !base.spaces.contains_key(&payload.id) {
        return Vec::new();
    }
    let Some(record) = base.space_conditions.get(&payload.id) else {
        return vec![ModelMutation::RemoveSpaceConditions(RemoveSpaceConditions { id: payload.id.clone() })];
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetSpaceConditions(SetSpaceConditions::from_patch(payload.id.clone(), restore))]
}
