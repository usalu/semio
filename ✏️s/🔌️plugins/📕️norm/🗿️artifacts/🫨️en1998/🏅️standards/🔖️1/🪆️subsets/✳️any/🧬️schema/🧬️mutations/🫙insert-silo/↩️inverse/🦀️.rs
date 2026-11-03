//! ↩️ `insert-silo` inverse: removes the inserted row again.

use crate::insert_silo::InsertSilo;
use crate::remove_silo::RemoveSilo;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &InsertSilo, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_silo::RemoveSilo> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertSilo, base: &En1998Snapshot) -> Option<RemoveSilo> {
    Some(RemoveSilo { id: payload.silo.id.clone() })
}
