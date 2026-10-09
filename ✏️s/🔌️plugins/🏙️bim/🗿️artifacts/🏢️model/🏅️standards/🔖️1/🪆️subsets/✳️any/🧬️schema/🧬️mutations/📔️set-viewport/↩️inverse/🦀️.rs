//! ↩️ Inverse of `SetViewport`: an absolute `SetViewport` restoring the base value of exactly the fields the forward really changes, none when the viewport is absent or nothing changes.

use super::SetViewport;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetViewport, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.viewports.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).negate(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetViewport(SetViewport::from_patch(payload.id.clone(), restore))]
}
