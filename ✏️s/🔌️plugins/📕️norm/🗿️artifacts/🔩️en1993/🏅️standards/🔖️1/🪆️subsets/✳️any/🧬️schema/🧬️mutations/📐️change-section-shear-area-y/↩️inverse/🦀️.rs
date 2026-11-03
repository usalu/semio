//! ↩️ `change-section-shear-area-y` inverse: restores the previous `shear_area_y`.

use crate::change_section_shear_area_y::ChangeSectionShearAreaY;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSectionShearAreaY, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_section_shear_area_y::ChangeSectionShearAreaY> {
    Ok(match base.sections.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSectionShearAreaY { id: payload.id.clone(), new_shear_area_y: row.shear_area_y })],
        None => Vec::new(),
    })
}
