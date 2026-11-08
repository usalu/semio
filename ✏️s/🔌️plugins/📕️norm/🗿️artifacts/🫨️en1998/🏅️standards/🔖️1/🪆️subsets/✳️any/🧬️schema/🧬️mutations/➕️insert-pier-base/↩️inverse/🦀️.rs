//! ↩️ `insert-pier-base` inverse: removes the inserted row again.

use crate::insert_pier_base::InsertPierBase;
use crate::remove_pier_base::RemovePierBase;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &InsertPierBase, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_pier_base::RemovePierBase> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertPierBase, base: &En1998Snapshot) -> Option<RemovePierBase> {
    let _ = base;
    Some(RemovePierBase { id: payload.pier_base.id.clone() })
}
