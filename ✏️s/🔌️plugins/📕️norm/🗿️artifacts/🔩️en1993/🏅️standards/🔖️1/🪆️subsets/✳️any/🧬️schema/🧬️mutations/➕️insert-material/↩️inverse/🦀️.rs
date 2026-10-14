use crate::insert_material::InsertMaterial;
use crate::remove_material;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &InsertMaterial, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_material::RemoveMaterial> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.materials.len());
    vec![Op::from(remove_material::RemoveMaterial { index: at })]

    })())
}
