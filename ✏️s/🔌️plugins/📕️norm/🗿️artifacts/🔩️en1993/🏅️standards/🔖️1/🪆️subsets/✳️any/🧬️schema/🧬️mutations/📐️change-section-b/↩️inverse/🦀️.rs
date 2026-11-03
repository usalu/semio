//! ↩️ `change-section-b` inverse: restores the previous `b`.

use crate::change_section_b::ChangeSectionB;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSectionB, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_section_b::ChangeSectionB> {
    Ok(match base.sections.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSectionB { id: payload.id.clone(), new_b: row.b })],
        None => Vec::new(),
    })
}
