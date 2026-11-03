//! ↩️ `remove-bridge-sls` inverse: inserts the removed row back at its position.

use crate::remove_bridge_sls::RemoveBridgeSls;
use crate::insert_bridge_sls::InsertBridgeSls;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &RemoveBridgeSls, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_bridge_sls::InsertBridgeSls> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveBridgeSls, base: &En1990Snapshot) -> Option<InsertBridgeSls> {
    let index = payload.index;
    let row = base.bridge_sls.get(index)?;
    Some(InsertBridgeSls { index: Some(index), item: row.clone() })
}
