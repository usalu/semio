//! ↩️ `insert-bridge-variable` inverse: removes the inserted row again.

use crate::insert_bridge_variable::InsertBridgeVariable;
use crate::remove_bridge_variable::RemoveBridgeVariable;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &InsertBridgeVariable, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_bridge_variable::RemoveBridgeVariable> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertBridgeVariable, base: &En1998Snapshot) -> Option<RemoveBridgeVariable> {
    let _r0 = base.bridges.iter().find(|row| row.id == payload.bridge_id)?;
    Some(RemoveBridgeVariable { bridge_id: payload.bridge_id.clone(), id: payload.variable.id.clone() })
}
