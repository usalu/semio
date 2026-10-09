//! ↩️ Inverse of `SetLeader`: an absolute `SetLeader` restoring the base value of exactly the fields the forward really changes, none when the leader is absent or nothing changes.

use super::SetLeader;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetLeader, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.leaders.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetLeader(SetLeader::from_patch(payload.id.clone(), restore))]
}
