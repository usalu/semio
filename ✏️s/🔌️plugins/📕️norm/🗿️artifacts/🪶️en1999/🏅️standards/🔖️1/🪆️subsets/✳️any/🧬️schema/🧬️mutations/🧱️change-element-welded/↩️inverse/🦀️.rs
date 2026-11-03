//! ↩️ `change-element-welded` inverse: restores the previous `welded`.

use crate::change_element_welded::ChangeElementWelded;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeElementWelded, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_element_welded::ChangeElementWelded> {
    Ok(match base.sections.iter().find(|row| row.id == payload.section_id).and_then(|row| row.elements.iter().find(|row| row.id == payload.element_id)) {
        Some(row) => vec![Op::from(ChangeElementWelded { section_id: payload.section_id.clone(), element_id: payload.element_id.clone(), new_welded: row.welded })],
        None => Vec::new(),
    })
}
