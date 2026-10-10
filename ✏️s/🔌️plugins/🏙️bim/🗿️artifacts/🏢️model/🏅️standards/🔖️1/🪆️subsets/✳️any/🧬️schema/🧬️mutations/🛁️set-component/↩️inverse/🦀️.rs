//! ↩️ Inverse of `SetComponent`: an absolute `SetComponent` restoring the base value of exactly the fields the forward really changes, none when the component is absent or nothing changes.

use super::SetComponent;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetComponent, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.components.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetComponent(SetComponent::from_patch(payload.id.clone(), restore))]
}
