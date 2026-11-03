//! ↩️ `change-section-area` inverse: restores the previous `area`.

use crate::change_section_area::ChangeSectionArea;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSectionArea, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_section_area::ChangeSectionArea> {
    Ok(match base.sections.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSectionArea { id: payload.id.clone(), new_area: row.area })],
        None => Vec::new(),
    })
}
