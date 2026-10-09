//! ↩️ Inverse of `SetTag`: an absolute `SetTag` restoring the base value of exactly the fields the forward really changes, none when the tag is absent or nothing changes.

use super::SetTag;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetTag, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.tags.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetTag(SetTag::from_patch(payload.id.clone(), restore))]
}
