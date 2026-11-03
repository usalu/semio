//! ↩️ `change-weld-run-element-id` inverse: restores the previous `element_id`.

use crate::change_weld_run_element_id::ChangeWeldRunElementId;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeWeldRunElementId, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_weld_run_element_id::ChangeWeldRunElementId> {
    Ok(match base.weld_runs.iter().find(|row| row.id == payload.weld_run_id) {
        Some(row) => vec![Op::from(ChangeWeldRunElementId { weld_run_id: payload.weld_run_id.clone(), new_element_id: row.element_id.clone() })],
        None => Vec::new(),
    })
}
