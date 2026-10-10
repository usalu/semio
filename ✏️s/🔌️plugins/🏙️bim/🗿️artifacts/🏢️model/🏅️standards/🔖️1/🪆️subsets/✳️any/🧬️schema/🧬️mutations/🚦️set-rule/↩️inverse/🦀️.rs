//! ↩️ Inverse of `SetRule`: an absolute `SetRule` restoring the base value of exactly the fields the forward really changes, none when the rule is absent or nothing changes.

use super::SetRule;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetRule, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.rules.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetRule(SetRule::from_patch(payload.id.clone(), restore))]
}
