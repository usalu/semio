//! ↩️ `change-bridge-second-lane-weight` inverse: restores the previous `second_lane_weight`.

use crate::change_bridge_second_lane_weight::ChangeBridgeSecondLaneWeight;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeSecondLaneWeight, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_second_lane_weight::ChangeBridgeSecondLaneWeight> {
    Ok(match base.bridge_fatigue.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBridgeSecondLaneWeight { id: payload.id.clone(), new_second_lane_weight: row.second_lane_weight })],
        None => Vec::new(),
    })
}
