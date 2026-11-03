//! ↩️ `change-element-stiffener-thk` inverse: restores the previous `edge_stiffener_thickness`.

use crate::change_element_stiffener_thk::ChangeElementStiffenerThk;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeElementStiffenerThk, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_element_stiffener_thk::ChangeElementStiffenerThk> {
    Ok(match base.sections.iter().find(|row| row.id == payload.section_id).and_then(|row| row.elements.iter().find(|row| row.id == payload.element_id)) {
        Some(row) => vec![Op::from(ChangeElementStiffenerThk { section_id: payload.section_id.clone(), element_id: payload.element_id.clone(), new_edge_stiffener_thickness: row.edge_stiffener_thickness })],
        None => Vec::new(),
    })
}
