//! ↩️ `insert-bridge-lane` inverse: removes the inserted row again.

use crate::insert_bridge_lane::InsertBridgeLane;
use crate::remove_bridge_lane::RemoveBridgeLane;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &InsertBridgeLane, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_bridge_lane::RemoveBridgeLane> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertBridgeLane, base: &En1991Snapshot) -> Option<RemoveBridgeLane> {
    Some(RemoveBridgeLane { id: payload.item.id.clone() })
}
