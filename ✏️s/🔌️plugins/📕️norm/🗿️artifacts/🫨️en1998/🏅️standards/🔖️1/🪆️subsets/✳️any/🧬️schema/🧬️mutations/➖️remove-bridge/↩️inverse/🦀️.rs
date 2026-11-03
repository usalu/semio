//! ↩️ `remove-bridge` inverse: inserts the removed row back at its position.

use crate::remove_bridge::RemoveBridge;
use crate::insert_bridge::InsertBridge;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &RemoveBridge, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_bridge::InsertBridge> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveBridge, base: &En1998Snapshot) -> Option<InsertBridge> {
    let (index, row) = base.bridges.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertBridge { index: Some(index), bridge: row.clone() })
}
