//! Inverse for `change-assumed-bridge-footway`.
use crate::change_assumed_bridge_footway::ChangeAssumedBridgeFootway;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeAssumedBridgeFootway, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assumed_bridge_footway::ChangeAssumedBridgeFootway> {
    Ok((|| {
    vec![Op::from(ChangeAssumedBridgeFootway { new_assumed_bridge_footway: base.assumed_bridge_footway })]

    })())
}
