use crate::insert_material_ductility::InsertMaterialDuctility;
use crate::remove_material_ductility;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &InsertMaterialDuctility, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_material_ductility::RemoveMaterialDuctility> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.material_ductility.len());
    vec![Op::from(remove_material_ductility::RemoveMaterialDuctility { index: at })]

    })())
}
