use super::InsertMaterial;
use crate::mutations::{remove_material, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &InsertMaterial, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    let at = payload.index.min(base.materials.len());
    vec![En1993Mutation::RemoveMaterial(remove_material::RemoveMaterial { index: at })]
}
