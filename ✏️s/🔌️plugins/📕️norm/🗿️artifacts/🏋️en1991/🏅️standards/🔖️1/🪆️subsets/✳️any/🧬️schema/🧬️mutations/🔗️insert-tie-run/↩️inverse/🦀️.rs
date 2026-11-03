//! ↩️ `insert-tie-run` inverse: removes the inserted row again.

use crate::insert_tie_run::InsertTieRun;
use crate::remove_tie_run::RemoveTieRun;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &InsertTieRun, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_tie_run::RemoveTieRun> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertTieRun, base: &En1991Snapshot) -> Option<RemoveTieRun> {
    Some(RemoveTieRun { id: payload.item.id.clone() })
}
