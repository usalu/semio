//! ↩️ `change-section-height` inverse: restores the previous `height`.

use crate::change_section_height::ChangeSectionHeight;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeSectionHeight, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_section_height::ChangeSectionHeight> {
    Ok(match base.sections.iter().find(|row| row.id == payload.section_id) {
        Some(row) => vec![Op::from(ChangeSectionHeight { section_id: payload.section_id.clone(), new_height: row.height })],
        None => Vec::new(),
    })
}
