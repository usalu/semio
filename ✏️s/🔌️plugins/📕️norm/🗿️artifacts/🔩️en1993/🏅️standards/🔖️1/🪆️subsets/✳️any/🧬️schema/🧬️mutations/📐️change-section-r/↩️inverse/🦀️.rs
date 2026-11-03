//! ↩️ `change-section-r` inverse: restores the previous `r`.

use crate::change_section_r::ChangeSectionR;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSectionR, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_section_r::ChangeSectionR> {
    Ok(match base.sections.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSectionR { id: payload.id.clone(), new_r: row.r })],
        None => Vec::new(),
    })
}
