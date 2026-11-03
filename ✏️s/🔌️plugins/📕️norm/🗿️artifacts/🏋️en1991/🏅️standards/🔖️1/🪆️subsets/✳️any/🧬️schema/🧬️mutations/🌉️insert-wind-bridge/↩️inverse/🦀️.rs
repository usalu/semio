//! ↩️ `insert-wind-bridge` inverse: removes the inserted row again.

use crate::insert_wind_bridge::InsertWindBridge;
use crate::remove_wind_bridge::RemoveWindBridge;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &InsertWindBridge, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_wind_bridge::RemoveWindBridge> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertWindBridge, base: &En1991Snapshot) -> Option<RemoveWindBridge> {
    Some(RemoveWindBridge { id: payload.item.id.clone() })
}
