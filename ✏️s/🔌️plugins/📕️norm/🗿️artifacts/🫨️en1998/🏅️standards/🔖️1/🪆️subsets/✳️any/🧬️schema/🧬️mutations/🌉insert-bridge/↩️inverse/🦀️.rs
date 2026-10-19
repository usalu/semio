//! ↩️ Inverse for `insert-bridge`.
use crate::insert_bridge::InsertBridge;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;
use crate::remove_bridge;

pub fn inverse<Op>(payload: &InsertBridge, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_bridge::RemoveBridge> {
    Ok((|| {
    vec![Op::from(remove_bridge::RemoveBridge { index: payload.index.unwrap_or(usize::MAX).min(base.bridges.len()) })]

    })())
}
