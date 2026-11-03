//! ↩️ `insert-bridge-sls` inverse: removes the inserted row again.

use crate::insert_bridge_sls::InsertBridgeSls;
use crate::remove_bridge_sls::RemoveBridgeSls;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &InsertBridgeSls, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_bridge_sls::RemoveBridgeSls> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertBridgeSls, base: &En1990Snapshot) -> Option<RemoveBridgeSls> {
    let index = payload.index.unwrap_or(usize::MAX).min(base.bridge_sls.len());
    Some(RemoveBridgeSls { index })
}
