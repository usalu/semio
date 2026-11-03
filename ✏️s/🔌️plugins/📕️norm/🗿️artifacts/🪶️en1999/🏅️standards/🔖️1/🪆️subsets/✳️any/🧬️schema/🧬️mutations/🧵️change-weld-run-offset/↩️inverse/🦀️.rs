//! ↩️ `change-weld-run-offset` inverse: restores the previous `offset`.

use crate::change_weld_run_offset::ChangeWeldRunOffset;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeWeldRunOffset, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_weld_run_offset::ChangeWeldRunOffset> {
    Ok(match base.weld_runs.iter().find(|row| row.id == payload.weld_run_id) {
        Some(row) => vec![Op::from(ChangeWeldRunOffset { weld_run_id: payload.weld_run_id.clone(), new_offset: row.offset })],
        None => Vec::new(),
    })
}
