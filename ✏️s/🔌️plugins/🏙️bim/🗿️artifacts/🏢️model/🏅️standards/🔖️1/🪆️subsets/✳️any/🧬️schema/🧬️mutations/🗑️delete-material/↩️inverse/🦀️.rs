//! ↩️ Inverse of `DeleteMaterial`: the concrete `CreateMaterial` carrying the full removed record, none when the material was absent.

use super::super::create_material::CreateMaterial;
use super::DeleteMaterial;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteMaterial, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.materials.get(&payload.id) {
        Some(material) => vec![ModelMutation::CreateMaterial(CreateMaterial { id: payload.id.clone(), material: material.clone() })],
        None => Vec::new(),
    }
}
