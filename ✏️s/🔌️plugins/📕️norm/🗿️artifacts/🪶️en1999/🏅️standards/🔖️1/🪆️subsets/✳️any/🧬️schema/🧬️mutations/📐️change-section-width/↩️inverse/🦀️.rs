//! ↩️ `change-section-width` inverse: restores the previous `width`.

use crate::change_section_width::ChangeSectionWidth;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeSectionWidth, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_section_width::ChangeSectionWidth> {
    Ok(match base.sections.iter().find(|row| row.id == payload.section_id) {
        Some(row) => vec![Op::from(ChangeSectionWidth { section_id: payload.section_id.clone(), new_width: row.width })],
        None => Vec::new(),
    })
}
