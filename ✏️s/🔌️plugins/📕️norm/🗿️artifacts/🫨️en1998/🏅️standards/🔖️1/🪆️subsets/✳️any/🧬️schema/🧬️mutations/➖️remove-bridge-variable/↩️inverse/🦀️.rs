//! ↩️ `remove-bridge-variable` inverse: inserts the removed row back at its position.

use crate::remove_bridge_variable::RemoveBridgeVariable;
use crate::insert_bridge_variable::InsertBridgeVariable;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &RemoveBridgeVariable, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_bridge_variable::InsertBridgeVariable> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveBridgeVariable, base: &En1998Snapshot) -> Option<InsertBridgeVariable> {
    let r0 = base.bridges.iter().find(|row| row.id == payload.bridge_id)?;
    let (index, row) = r0.variables.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertBridgeVariable { bridge_id: payload.bridge_id.clone(), index: Some(index), variable: row.clone() })
}
