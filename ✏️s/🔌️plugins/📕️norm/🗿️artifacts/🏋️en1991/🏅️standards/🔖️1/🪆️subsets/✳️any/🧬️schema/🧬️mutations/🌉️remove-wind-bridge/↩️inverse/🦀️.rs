//! ↩️ `remove-wind-bridge` inverse: inserts the removed row back at its position.

use crate::remove_wind_bridge::RemoveWindBridge;
use crate::insert_wind_bridge::InsertWindBridge;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &RemoveWindBridge, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_wind_bridge::InsertWindBridge> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveWindBridge, base: &En1991Snapshot) -> Option<InsertWindBridge> {
    let (index, row) = base.wind_bridges.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertWindBridge { index, item: row.clone() })
}
