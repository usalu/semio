//! ↩️ `change-weld-run-start` inverse: restores the previous `start`.

use crate::change_weld_run_start::ChangeWeldRunStart;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeWeldRunStart, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_weld_run_start::ChangeWeldRunStart> {
    Ok(match base.weld_runs.iter().find(|row| row.id == payload.weld_run_id) {
        Some(row) => vec![Op::from(ChangeWeldRunStart { weld_run_id: payload.weld_run_id.clone(), new_start: row.start })],
        None => Vec::new(),
    })
}
