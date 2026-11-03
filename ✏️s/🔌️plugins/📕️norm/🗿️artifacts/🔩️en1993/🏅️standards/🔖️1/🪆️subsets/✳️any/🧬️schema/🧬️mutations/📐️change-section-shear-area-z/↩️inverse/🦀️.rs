//! ↩️ `change-section-shear-area-z` inverse: restores the previous `shear_area_z`.

use crate::change_section_shear_area_z::ChangeSectionShearAreaZ;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSectionShearAreaZ, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_section_shear_area_z::ChangeSectionShearAreaZ> {
    Ok(match base.sections.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSectionShearAreaZ { id: payload.id.clone(), new_shear_area_z: row.shear_area_z })],
        None => Vec::new(),
    })
}
