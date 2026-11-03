//! ↩️ `insert-bearing` inverse: removes the inserted row again.

use crate::insert_bearing::InsertBearing;
use crate::remove_bearing::RemoveBearing;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &InsertBearing, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_bearing::RemoveBearing> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertBearing, base: &En1998Snapshot) -> Option<RemoveBearing> {
    Some(RemoveBearing { id: payload.bearing.id.clone() })
}
