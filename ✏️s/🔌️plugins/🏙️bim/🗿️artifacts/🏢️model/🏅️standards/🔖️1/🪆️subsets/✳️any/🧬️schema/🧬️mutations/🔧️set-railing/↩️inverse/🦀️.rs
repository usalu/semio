//! ↩️ Inverse of `SetRailing`: an absolute `SetRailing` restoring the base value of exactly the fields the forward really changes, none when the railing is absent or nothing changes.

use super::SetRailing;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetRailing, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.railings.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetRailing(SetRailing::from_patch(payload.id.clone(), restore))]
}
