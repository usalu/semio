//! ↩️ Inverse for `change-assumed-bridge-tandem`.
use crate::change_assumed_bridge_tandem::ChangeAssumedBridgeTandem;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeAssumedBridgeTandem, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assumed_bridge_tandem::ChangeAssumedBridgeTandem> {
    Ok((|| {
    vec![Op::from(ChangeAssumedBridgeTandem { new_assumed_bridge_tandem: base.assumed_bridge_tandem })]

    })())
}
