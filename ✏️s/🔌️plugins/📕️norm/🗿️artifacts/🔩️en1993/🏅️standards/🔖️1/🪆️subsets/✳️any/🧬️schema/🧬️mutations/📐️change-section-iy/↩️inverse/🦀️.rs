//! ↩️ `change-section-iy` inverse: restores the previous `iy`.

use crate::change_section_iy::ChangeSectionIy;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSectionIy, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_section_iy::ChangeSectionIy> {
    Ok(match base.sections.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSectionIy { id: payload.id.clone(), new_iy: row.iy })],
        None => Vec::new(),
    })
}
