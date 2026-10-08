//! ↩️ Inverse of `SetProjectInfo`: one `SetProjectInfo` restoring the base value of exactly the fields the forward really changes, none when nothing changes.

use super::SetProjectInfo;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetProjectInfo, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let restore = payload.patch().minimal(&base.project).negate(&base.project);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetProjectInfo(SetProjectInfo::from_patch(restore))]
}
