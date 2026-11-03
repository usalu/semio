//! ↩️ `insert-bridge` inverse: removes the inserted row again.

use crate::insert_bridge::InsertBridge;
use crate::remove_bridge::RemoveBridge;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &InsertBridge, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_bridge::RemoveBridge> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertBridge, base: &En1998Snapshot) -> Option<RemoveBridge> {
    Some(RemoveBridge { id: payload.bridge.id.clone() })
}
