//! ↩️ `change-weld-run-direction` inverse: restores the previous `direction`.

use crate::change_weld_run_direction::ChangeWeldRunDirection;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeWeldRunDirection, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_weld_run_direction::ChangeWeldRunDirection> {
    Ok(match base.weld_runs.iter().find(|row| row.id == payload.weld_run_id) {
        Some(row) => vec![Op::from(ChangeWeldRunDirection { weld_run_id: payload.weld_run_id.clone(), new_direction: row.direction })],
        None => Vec::new(),
    })
}
