//! ↩️ Inverse of `SetBuilding`: an absolute `SetBuilding` restoring the base value of exactly the fields the forward really changes, none when the building is absent or nothing changes.

use super::SetBuilding;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetBuilding, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.buildings.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).negate(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetBuilding(SetBuilding::from_patch(payload.id.clone(), restore))]
}
