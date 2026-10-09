//! ↩️ Inverse of `SetFamily`: an absolute `SetFamily` restoring the base value of exactly the fields the forward really changes, none when the family is absent or nothing changes.

use super::SetFamily;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetFamily, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.families.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetFamily(SetFamily::from_patch(payload.id.clone(), restore))]
}
