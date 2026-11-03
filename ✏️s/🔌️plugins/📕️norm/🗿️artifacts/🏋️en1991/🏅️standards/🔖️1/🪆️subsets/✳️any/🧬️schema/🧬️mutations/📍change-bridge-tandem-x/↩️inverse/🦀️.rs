//! ↩️ Inverse for `change-bridge-tandem-x`.
use crate::change_bridge_tandem_x::ChangeBridgeTandemX;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeBridgeTandemX, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_tandem_x::ChangeBridgeTandemX> {
    Ok((|| {
    vec![Op::from(ChangeBridgeTandemX { new_bridge_tandem_x: base.bridge_tandem_x })]

    })())
}
