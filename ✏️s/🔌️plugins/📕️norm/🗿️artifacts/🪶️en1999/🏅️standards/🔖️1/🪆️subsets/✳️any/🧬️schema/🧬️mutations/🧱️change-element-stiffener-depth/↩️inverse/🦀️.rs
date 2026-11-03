//! ↩️ `change-element-stiffener-depth` inverse: restores the previous `edge_stiffener_depth`.

use crate::change_element_stiffener_depth::ChangeElementStiffenerDepth;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeElementStiffenerDepth, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_element_stiffener_depth::ChangeElementStiffenerDepth> {
    Ok(match base.sections.iter().find(|row| row.id == payload.section_id).and_then(|row| row.elements.iter().find(|row| row.id == payload.element_id)) {
        Some(row) => vec![Op::from(ChangeElementStiffenerDepth { section_id: payload.section_id.clone(), element_id: payload.element_id.clone(), new_edge_stiffener_depth: row.edge_stiffener_depth })],
        None => Vec::new(),
    })
}
