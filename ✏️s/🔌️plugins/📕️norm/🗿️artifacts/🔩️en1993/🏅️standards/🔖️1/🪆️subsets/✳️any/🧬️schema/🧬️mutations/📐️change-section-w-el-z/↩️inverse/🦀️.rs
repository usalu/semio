//! ↩️ `change-section-w-el-z` inverse: restores the previous `w_el_z`.

use crate::change_section_w_el_z::ChangeSectionWElZ;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSectionWElZ, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_section_w_el_z::ChangeSectionWElZ> {
    Ok(match base.sections.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSectionWElZ { id: payload.id.clone(), new_w_el_z: row.w_el_z })],
        None => Vec::new(),
    })
}
