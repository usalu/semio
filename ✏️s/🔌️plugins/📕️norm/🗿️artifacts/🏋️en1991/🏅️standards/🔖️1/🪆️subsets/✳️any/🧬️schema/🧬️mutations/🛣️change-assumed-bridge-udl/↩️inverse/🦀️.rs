//! ↩️ Inverse for `change-assumed-bridge-udl`.
use crate::change_assumed_bridge_udl::ChangeAssumedBridgeUdl;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeAssumedBridgeUdl, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assumed_bridge_udl::ChangeAssumedBridgeUdl> {
    Ok((|| {
    vec![Op::from(ChangeAssumedBridgeUdl { new_assumed_bridge_udl: base.assumed_bridge_udl })]

    })())
}
