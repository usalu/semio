//! ↩️ Inverse of `SetClassificationSystem`: an absolute `SetClassificationSystem` restoring the base value of exactly the fields the forward really changes (a removed source assigned null), none when the system is absent or
//! nothing changes.

use super::SetClassificationSystem;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetClassificationSystem, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.classification_systems.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetClassificationSystem(SetClassificationSystem::from_patch(payload.id.clone(), restore))]
}
