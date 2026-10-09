//! ↩️ Inverse of `SetCeiling`: an absolute `SetCeiling` restoring the base value of exactly the fields the forward really changes, none when the ceiling is absent or nothing changes.

use super::SetCeiling;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetCeiling, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.ceilings.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetCeiling(SetCeiling::from_patch(payload.id.clone(), restore))]
}
