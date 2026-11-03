//! ↩️ `change-param-section-id` inverse: restores the previous `section_id`.

use crate::change_param_section_id::ChangeParamSectionId;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeParamSectionId, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_param_section_id::ChangeParamSectionId> {
    Ok(match base.parametric_sections.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeParamSectionId { id: payload.id.clone(), new_section_id: row.section_id.clone() })],
        None => Vec::new(),
    })
}
