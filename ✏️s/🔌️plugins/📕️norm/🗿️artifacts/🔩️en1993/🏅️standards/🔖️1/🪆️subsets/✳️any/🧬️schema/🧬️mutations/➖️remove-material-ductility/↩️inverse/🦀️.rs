use crate::remove_material_ductility::RemoveMaterialDuctility;
use crate::insert_material_ductility;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &RemoveMaterialDuctility, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_material_ductility::InsertMaterialDuctility> {
    Ok((|| {
    if payload.index >= base.material_ductility.len() { return Vec::new(); }
    vec![Op::from(insert_material_ductility::InsertMaterialDuctility { index: Some(payload.index), material_ductility: base.material_ductility[payload.index].clone() })]

    })())
}
