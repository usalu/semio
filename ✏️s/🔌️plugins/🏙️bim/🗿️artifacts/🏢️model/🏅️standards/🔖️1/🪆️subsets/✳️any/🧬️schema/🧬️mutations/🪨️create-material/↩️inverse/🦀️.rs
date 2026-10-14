//! ↩️ Inverse of `CreateMaterial`: the concrete `DeleteMaterial` of the id it created, none when the id was already taken.

use super::super::delete_material::DeleteMaterial;
use super::CreateMaterial;
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot};

pub fn inverse(payload: &CreateMaterial, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.materials.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteMaterial(DeleteMaterial { id: payload.id.clone() })]
}
