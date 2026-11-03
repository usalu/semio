//! ↩️ `change-element-width` inverse: restores the previous `width`.

use crate::change_element_width::ChangeElementWidth;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeElementWidth, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_element_width::ChangeElementWidth> {
    Ok(match base.sections.iter().find(|row| row.id == payload.section_id).and_then(|row| row.elements.iter().find(|row| row.id == payload.element_id)) {
        Some(row) => vec![Op::from(ChangeElementWidth { section_id: payload.section_id.clone(), element_id: payload.element_id.clone(), new_width: row.width })],
        None => Vec::new(),
    })
}
