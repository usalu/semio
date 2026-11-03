//! ↩️ `change-section-h` inverse: restores the previous `h`.

use crate::change_section_h::ChangeSectionH;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSectionH, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_section_h::ChangeSectionH> {
    Ok(match base.sections.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSectionH { id: payload.id.clone(), new_h: row.h })],
        None => Vec::new(),
    })
}
