use super::RemoveMaterial;
use crate::mutations::{insert_material, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &RemoveMaterial, base: &En1993Snapshot) -> Result<Vec<En1993Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if payload.index >= base.materials.len() { return Vec::new(); }
    vec![En1993Mutation::InsertMaterial(insert_material::InsertMaterial { index: Some(payload.index), material: base.materials[payload.index].clone() })]

    })())
}
