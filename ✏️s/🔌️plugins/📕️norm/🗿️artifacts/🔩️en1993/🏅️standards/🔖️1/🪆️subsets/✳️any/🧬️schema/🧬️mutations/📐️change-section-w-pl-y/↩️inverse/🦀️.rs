//! ↩️ `change-section-w-pl-y` inverse: restores the previous `w_pl_y`.

use crate::change_section_w_pl_y::ChangeSectionWPlY;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSectionWPlY, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_section_w_pl_y::ChangeSectionWPlY> {
    Ok(match base.sections.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSectionWPlY { id: payload.id.clone(), new_w_pl_y: row.w_pl_y })],
        None => Vec::new(),
    })
}
