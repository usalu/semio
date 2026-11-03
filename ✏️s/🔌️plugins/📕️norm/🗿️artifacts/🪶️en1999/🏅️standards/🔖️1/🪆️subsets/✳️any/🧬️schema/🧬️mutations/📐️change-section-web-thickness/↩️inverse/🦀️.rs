//! ↩️ `change-section-web-thickness` inverse: restores the previous `web_thickness`.

use crate::change_section_web_thickness::ChangeSectionWebThickness;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeSectionWebThickness, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_section_web_thickness::ChangeSectionWebThickness> {
    Ok(match base.sections.iter().find(|row| row.id == payload.section_id) {
        Some(row) => vec![Op::from(ChangeSectionWebThickness { section_id: payload.section_id.clone(), new_web_thickness: row.web_thickness })],
        None => Vec::new(),
    })
}
