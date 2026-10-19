//! ↩️ Inverse for `change-bridge-lane-width`.
use crate::change_bridge_lane_width::ChangeBridgeLaneWidth;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeBridgeLaneWidth, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_lane_width::ChangeBridgeLaneWidth> {
    Ok((|| {
    vec![Op::from(ChangeBridgeLaneWidth { new_bridge_lane_width: base.bridge_lane_width })]

    })())
}
