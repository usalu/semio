//! ↩️ `change-weld-run-haz-extent` inverse: restores the previous `haz_extent`.

use crate::change_weld_run_haz_extent::ChangeWeldRunHazExtent;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeWeldRunHazExtent, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_weld_run_haz_extent::ChangeWeldRunHazExtent> {
    Ok(match base.weld_runs.iter().find(|row| row.id == payload.weld_run_id) {
        Some(row) => vec![Op::from(ChangeWeldRunHazExtent { weld_run_id: payload.weld_run_id.clone(), new_haz_extent: row.haz_extent })],
        None => Vec::new(),
    })
}
