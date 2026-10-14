//! ↩️ Inverse of `SetMaterial`: an absolute `SetMaterial` restoring the base value of exactly the fields the forward really changes, none when the material is absent or nothing changes.

use super::SetMaterial;
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot, diff::Patch};

pub fn inverse(payload: &SetMaterial, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.materials.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetMaterial(SetMaterial::from_patch(payload.id.clone(), restore))]
}
