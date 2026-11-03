//! ↩️ `change-section-w-pl-z` inverse: restores the previous `w_pl_z`.

use crate::change_section_w_pl_z::ChangeSectionWPlZ;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSectionWPlZ, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_section_w_pl_z::ChangeSectionWPlZ> {
    Ok(match base.sections.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSectionWPlZ { id: payload.id.clone(), new_w_pl_z: row.w_pl_z })],
        None => Vec::new(),
    })
}
