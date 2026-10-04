//! Inverse for `change-bridge-lane-width`.
use super::ChangeBridgeLaneWidth;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeBridgeLaneWidth, base: &En1991Snapshot) -> Result<Vec<En1991Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1991Mutation::ChangeBridgeLaneWidth(ChangeBridgeLaneWidth { new_bridge_lane_width: base.bridge_lane_width })]

    })())
}
