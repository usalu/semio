//! ↩️ `change-section-iz` inverse: restores the previous `iz`.

use crate::change_section_iz::ChangeSectionIz;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSectionIz, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_section_iz::ChangeSectionIz> {
    Ok(match base.sections.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSectionIz { id: payload.id.clone(), new_iz: row.iz })],
        None => Vec::new(),
    })
}
