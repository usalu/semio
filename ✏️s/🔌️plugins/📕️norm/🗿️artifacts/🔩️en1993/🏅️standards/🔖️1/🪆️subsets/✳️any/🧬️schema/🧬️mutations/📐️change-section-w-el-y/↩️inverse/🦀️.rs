//! ↩️ `change-section-w-el-y` inverse: restores the previous `w_el_y`.

use crate::change_section_w_el_y::ChangeSectionWElY;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSectionWElY, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_section_w_el_y::ChangeSectionWElY> {
    Ok(match base.sections.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSectionWElY { id: payload.id.clone(), new_w_el_y: row.w_el_y })],
        None => Vec::new(),
    })
}
