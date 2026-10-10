//! ↩️ Inverse of `SetClashSet`: an absolute `SetClashSet` restoring the base value of exactly the fields the forward really changes, none when the clash set is absent or nothing changes.

use super::SetClashSet;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetClashSet, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.clash_sets.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetClashSet(SetClashSet::from_patch(payload.id.clone(), restore))]
}
