//! ↩️ Inverse of `SetFamilySolid`: an absolute `SetFamilySolid` restoring the base value of exactly the fields the forward really changes, none when the solid is absent or nothing changes.

use super::SetFamilySolid;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetFamilySolid, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.family_solids.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetFamilySolid(SetFamilySolid::from_patch(payload.id.clone(), restore))]
}
