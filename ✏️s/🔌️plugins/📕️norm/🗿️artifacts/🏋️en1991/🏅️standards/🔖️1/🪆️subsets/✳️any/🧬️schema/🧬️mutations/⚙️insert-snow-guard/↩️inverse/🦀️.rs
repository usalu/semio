//! ↩️ `insert-snow-guard` inverse: removes the inserted row again.

use crate::insert_snow_guard::InsertSnowGuard;
use crate::remove_snow_guard::RemoveSnowGuard;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &InsertSnowGuard, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_snow_guard::RemoveSnowGuard> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertSnowGuard, base: &En1991Snapshot) -> Option<RemoveSnowGuard> {
    Some(RemoveSnowGuard { id: payload.item.id.clone() })
}
