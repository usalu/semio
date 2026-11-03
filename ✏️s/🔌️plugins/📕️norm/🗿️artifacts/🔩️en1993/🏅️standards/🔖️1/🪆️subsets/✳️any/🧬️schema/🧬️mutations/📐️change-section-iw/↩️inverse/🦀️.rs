//! ↩️ `change-section-iw` inverse: restores the previous `iw`.

use crate::change_section_iw::ChangeSectionIw;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSectionIw, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_section_iw::ChangeSectionIw> {
    Ok(match base.sections.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSectionIw { id: payload.id.clone(), new_iw: row.iw })],
        None => Vec::new(),
    })
}
