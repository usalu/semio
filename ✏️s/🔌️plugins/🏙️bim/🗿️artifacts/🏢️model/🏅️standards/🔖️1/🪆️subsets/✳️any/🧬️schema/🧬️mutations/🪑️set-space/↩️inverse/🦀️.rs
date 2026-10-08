//! ↩️ Inverse of `SetSpace`: an absolute `SetSpace` restoring the base value of exactly the fields the forward really changes, none when the space is absent or nothing changes.

use super::SetSpace;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetSpace, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.spaces.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetSpace(SetSpace::from_patch(payload.id.clone(), restore))]
}
