//! ↩️ Inverse of `SetIssue`: an absolute `SetIssue` restoring the base value of exactly the fields the forward really changes, none when the issue is absent or nothing changes.

use super::SetIssue;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetIssue, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.issues.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetIssue(SetIssue::from_patch(payload.id.clone(), restore))]
}
