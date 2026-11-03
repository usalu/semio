//! ↩️ `remove-tie-run` inverse: inserts the removed row back at its position.

use crate::remove_tie_run::RemoveTieRun;
use crate::insert_tie_run::InsertTieRun;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &RemoveTieRun, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_tie_run::InsertTieRun> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveTieRun, base: &En1991Snapshot) -> Option<InsertTieRun> {
    let (index, row) = base.tie_runs.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertTieRun { index, item: row.clone() })
}
