//! ↩️ `change-element-weld-process` inverse: restores the previous `weld_process`.

use crate::change_element_weld_process::ChangeElementWeldProcess;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeElementWeldProcess, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_element_weld_process::ChangeElementWeldProcess> {
    Ok(match base.sections.iter().find(|row| row.id == payload.section_id).and_then(|row| row.elements.iter().find(|row| row.id == payload.element_id)) {
        Some(row) => vec![Op::from(ChangeElementWeldProcess { section_id: payload.section_id.clone(), element_id: payload.element_id.clone(), new_weld_process: row.weld_process })],
        None => Vec::new(),
    })
}
