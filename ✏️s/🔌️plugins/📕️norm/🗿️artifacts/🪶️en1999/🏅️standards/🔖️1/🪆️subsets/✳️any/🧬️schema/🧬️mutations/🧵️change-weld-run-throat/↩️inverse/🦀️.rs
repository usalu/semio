//! ↩️ `change-weld-run-throat` inverse: restores the previous `throat`.

use crate::change_weld_run_throat::ChangeWeldRunThroat;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeWeldRunThroat, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_weld_run_throat::ChangeWeldRunThroat> {
    Ok(match base.weld_runs.iter().find(|row| row.id == payload.weld_run_id) {
        Some(row) => vec![Op::from(ChangeWeldRunThroat { weld_run_id: payload.weld_run_id.clone(), new_throat: row.throat })],
        None => Vec::new(),
    })
}
