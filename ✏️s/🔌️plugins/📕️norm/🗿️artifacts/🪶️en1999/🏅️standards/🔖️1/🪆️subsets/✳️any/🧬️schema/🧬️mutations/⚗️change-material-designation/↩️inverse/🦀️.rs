//! ↩️ `change-material-designation` inverse.

use crate::change_material_designation::ChangeMaterialDesignation;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeMaterialDesignation, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_material_designation::ChangeMaterialDesignation> {
    Ok((|| {
    let d = base.materials.iter().find(|m| m.id == payload.material_id).map(|m| m.designation.clone()).unwrap_or_default();
    vec![Op::from(ChangeMaterialDesignation { material_id: payload.material_id.clone(), new_designation: d })]

    })())
}
