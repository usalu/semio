//! ↩️ `remove-bridge-lane` inverse: inserts the removed row back at its position.

use crate::remove_bridge_lane::RemoveBridgeLane;
use crate::insert_bridge_lane::InsertBridgeLane;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &RemoveBridgeLane, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_bridge_lane::InsertBridgeLane> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveBridgeLane, base: &En1991Snapshot) -> Option<InsertBridgeLane> {
    let (index, row) = base.bridge_lanes.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertBridgeLane { index, item: row.clone() })
}
