//! Inverse for `change-assumed-bridge-lm4`.
use crate::change_assumed_bridge_lm4::ChangeAssumedBridgeLm4;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeAssumedBridgeLm4, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assumed_bridge_lm4::ChangeAssumedBridgeLm4> {
    Ok((|| {
    vec![Op::from(ChangeAssumedBridgeLm4 { new_assumed_bridge_lm4: base.assumed_bridge_lm4 })]

    })())
}
