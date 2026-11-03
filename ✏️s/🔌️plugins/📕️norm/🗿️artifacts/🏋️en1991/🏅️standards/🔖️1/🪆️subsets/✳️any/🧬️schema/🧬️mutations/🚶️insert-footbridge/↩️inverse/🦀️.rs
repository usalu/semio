//! ↩️ `insert-footbridge` inverse: removes the inserted row again.

use crate::insert_footbridge::InsertFootbridge;
use crate::remove_footbridge::RemoveFootbridge;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &InsertFootbridge, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_footbridge::RemoveFootbridge> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertFootbridge, base: &En1991Snapshot) -> Option<RemoveFootbridge> {
    Some(RemoveFootbridge { id: payload.item.id.clone() })
}
