//! ↩️ Inverse of `SetRoofType`: an absolute `SetRoofType` restoring the base value of exactly the fields the forward really changes, none when the roof type is absent or nothing changes.

use super::SetRoofType;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetRoofType, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.roof_types.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetRoofType(SetRoofType::from_patch(payload.id.clone(), restore))]
}
