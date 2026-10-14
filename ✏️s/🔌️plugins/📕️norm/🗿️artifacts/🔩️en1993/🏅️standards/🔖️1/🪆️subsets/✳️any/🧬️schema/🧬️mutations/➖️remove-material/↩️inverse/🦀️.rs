use crate::remove_material::RemoveMaterial;
use crate::insert_material;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &RemoveMaterial, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_material::InsertMaterial> {
    Ok((|| {
    if payload.index >= base.materials.len() { return Vec::new(); }
    vec![Op::from(insert_material::InsertMaterial { index: Some(payload.index), material: base.materials[payload.index].clone() })]

    })())
}
