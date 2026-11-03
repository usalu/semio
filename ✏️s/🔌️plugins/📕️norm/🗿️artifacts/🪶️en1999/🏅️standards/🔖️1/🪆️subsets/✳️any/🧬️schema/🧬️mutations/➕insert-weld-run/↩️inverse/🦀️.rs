//! ↩️ `insert-weld-run` inverse.

use crate::insert_weld_run::InsertWeldRun;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::remove_weld_run;

pub fn inverse<Op>(payload: &InsertWeldRun, _base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_weld_run::RemoveWeldRun> {
    Ok((|| {
    vec![Op::from(remove_weld_run::RemoveWeldRun { id: payload.weld_run.id.clone() })]

    })())
}
