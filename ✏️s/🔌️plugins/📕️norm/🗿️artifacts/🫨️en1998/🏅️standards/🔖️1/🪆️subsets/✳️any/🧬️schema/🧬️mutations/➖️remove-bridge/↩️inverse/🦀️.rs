//! Inverse for `remove-bridge`.
use crate::remove_bridge::RemoveBridge;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;
use crate::insert_bridge;

pub fn inverse<Op>(payload: &RemoveBridge, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_bridge::InsertBridge> {
    Ok((|| {
    match base.bridges.get(payload.index) {
        Some(item) => vec![Op::from(insert_bridge::InsertBridge { index: Some(payload.index), bridge: item.clone() })],
        None => Vec::new(),
    }

    })())
}
