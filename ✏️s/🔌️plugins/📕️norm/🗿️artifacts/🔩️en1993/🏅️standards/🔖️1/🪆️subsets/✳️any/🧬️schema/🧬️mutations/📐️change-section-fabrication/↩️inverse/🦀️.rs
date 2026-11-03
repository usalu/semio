//! ↩️ `change-section-fabrication` inverse: restores the previous `fabrication`.

use crate::change_section_fabrication::ChangeSectionFabrication;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSectionFabrication, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_section_fabrication::ChangeSectionFabrication> {
    Ok(match base.sections.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSectionFabrication { id: payload.id.clone(), new_fabrication: row.fabrication })],
        None => Vec::new(),
    })
}
