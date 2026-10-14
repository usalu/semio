//! Inverse for `change-assumed-bridge-lm3`.
use crate::change_assumed_bridge_lm3::ChangeAssumedBridgeLm3;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeAssumedBridgeLm3, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assumed_bridge_lm3::ChangeAssumedBridgeLm3> {
    Ok((|| {
    vec![Op::from(ChangeAssumedBridgeLm3 { new_assumed_bridge_lm3: base.assumed_bridge_lm3 })]

    })())
}
