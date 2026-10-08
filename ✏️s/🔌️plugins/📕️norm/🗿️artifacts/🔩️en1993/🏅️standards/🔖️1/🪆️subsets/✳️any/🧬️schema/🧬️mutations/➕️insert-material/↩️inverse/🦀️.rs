use super::InsertMaterial;
use crate::mutations::{remove_material, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &InsertMaterial, base: &En1993Snapshot) -> Result<Vec<En1993Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.materials.len());
    vec![En1993Mutation::RemoveMaterial(remove_material::RemoveMaterial { index: at })]

    })())
}
