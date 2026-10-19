//! ↩️ Inverse for `change-assumed-bridge-lm2`.
use crate::change_assumed_bridge_lm2::ChangeAssumedBridgeLm2;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeAssumedBridgeLm2, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assumed_bridge_lm2::ChangeAssumedBridgeLm2> {
    Ok((|| {
    vec![Op::from(ChangeAssumedBridgeLm2 { new_assumed_bridge_lm2: base.assumed_bridge_lm2 })]

    })())
}
