//! ↩️ `change-weld-run-length` inverse: restores the previous `length`.

use crate::change_weld_run_length::ChangeWeldRunLength;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeWeldRunLength, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_weld_run_length::ChangeWeldRunLength> {
    Ok(match base.weld_runs.iter().find(|row| row.id == payload.weld_run_id) {
        Some(row) => vec![Op::from(ChangeWeldRunLength { weld_run_id: payload.weld_run_id.clone(), new_length: row.length })],
        None => Vec::new(),
    })
}
