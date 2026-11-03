//! ↩️ `remove-weld-run` inverse.

use crate::remove_weld_run::RemoveWeldRun;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::insert_weld_run;

pub fn inverse<Op>(payload: &RemoveWeldRun, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_weld_run::InsertWeldRun> {
    Ok((|| {
    if let Some((index, row)) = base.weld_runs.iter().enumerate().find(|(_, row)| row.id == payload.id) {
        vec![Op::from(insert_weld_run::InsertWeldRun { index: index as u32, weld_run: row.clone() })]
    } else { Vec::new() }

    })())
}
