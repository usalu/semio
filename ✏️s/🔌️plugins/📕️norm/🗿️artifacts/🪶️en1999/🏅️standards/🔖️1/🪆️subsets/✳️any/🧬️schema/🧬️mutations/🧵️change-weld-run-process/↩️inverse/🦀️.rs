//! ↩️ `change-weld-run-process` inverse: restores the previous `process`.

use crate::change_weld_run_process::ChangeWeldRunProcess;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeWeldRunProcess, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_weld_run_process::ChangeWeldRunProcess> {
    Ok(match base.weld_runs.iter().find(|row| row.id == payload.weld_run_id) {
        Some(row) => vec![Op::from(ChangeWeldRunProcess { weld_run_id: payload.weld_run_id.clone(), new_process: row.process })],
        None => Vec::new(),
    })
}
