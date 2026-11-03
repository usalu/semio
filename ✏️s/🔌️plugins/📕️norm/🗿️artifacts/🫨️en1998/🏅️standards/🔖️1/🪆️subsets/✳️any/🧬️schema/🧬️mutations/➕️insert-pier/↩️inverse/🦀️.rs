//! ↩️ `insert-pier` inverse: removes the inserted row again.

use crate::insert_pier::InsertPier;
use crate::remove_pier::RemovePier;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &InsertPier, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_pier::RemovePier> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertPier, base: &En1998Snapshot) -> Option<RemovePier> {
    Some(RemovePier { id: payload.pier.id.clone() })
}
