//! ↩️ `change-weld-run-t-junction` inverse: restores the previous `t_junction`.

use crate::change_weld_run_t_junction::ChangeWeldRunTJunction;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeWeldRunTJunction, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_weld_run_t_junction::ChangeWeldRunTJunction> {
    Ok(match base.weld_runs.iter().find(|row| row.id == payload.weld_run_id) {
        Some(row) => vec![Op::from(ChangeWeldRunTJunction { weld_run_id: payload.weld_run_id.clone(), new_t_junction: row.t_junction })],
        None => Vec::new(),
    })
}
