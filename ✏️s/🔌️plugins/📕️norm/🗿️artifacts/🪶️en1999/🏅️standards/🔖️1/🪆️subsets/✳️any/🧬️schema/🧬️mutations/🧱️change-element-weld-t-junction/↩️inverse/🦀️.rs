//! ↩️ `change-element-weld-t-junction` inverse: restores the previous `weld_t_junction`.

use crate::change_element_weld_t_junction::ChangeElementWeldTJunction;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeElementWeldTJunction, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_element_weld_t_junction::ChangeElementWeldTJunction> {
    Ok(match base.sections.iter().find(|row| row.id == payload.section_id).and_then(|row| row.elements.iter().find(|row| row.id == payload.element_id)) {
        Some(row) => vec![Op::from(ChangeElementWeldTJunction { section_id: payload.section_id.clone(), element_id: payload.element_id.clone(), new_weld_t_junction: row.weld_t_junction })],
        None => Vec::new(),
    })
}
