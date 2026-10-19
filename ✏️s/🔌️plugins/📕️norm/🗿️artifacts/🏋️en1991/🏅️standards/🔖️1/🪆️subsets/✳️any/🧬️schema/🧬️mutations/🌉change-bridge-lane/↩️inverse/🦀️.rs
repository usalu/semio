//! ↩️ Inverse for `change-bridge-lane`.
use crate::change_bridge_lane::ChangeBridgeLane;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeBridgeLane, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_lane::ChangeBridgeLane> {
    Ok((|| {
    vec![Op::from(ChangeBridgeLane { new_bridge_lane: base.bridge_lane })]

    })())
}
