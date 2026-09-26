use super::RemoveMaterial;
use crate::mutations::{insert_material, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &RemoveMaterial, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    if payload.index >= base.materials.len() { return Vec::new(); }
    vec![En1993Mutation::InsertMaterial(insert_material::InsertMaterial { index: payload.index, material: base.materials[payload.index].clone() })]
}
