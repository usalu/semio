//! ↩️ `insert-wind-face` inverse: removes the inserted row again.

use crate::insert_wind_face::InsertWindFace;
use crate::remove_wind_face::RemoveWindFace;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &InsertWindFace, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_wind_face::RemoveWindFace> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertWindFace, base: &En1991Snapshot) -> Option<RemoveWindFace> {
    Some(RemoveWindFace { id: payload.item.id.clone() })
}
