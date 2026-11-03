//! ↩️ `change-section-designation` inverse: restores the previous `designation`.

use crate::change_section_designation::ChangeSectionDesignation;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSectionDesignation, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_section_designation::ChangeSectionDesignation> {
    Ok(match base.sections.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSectionDesignation { id: payload.id.clone(), new_designation: row.designation.clone() })],
        None => Vec::new(),
    })
}
