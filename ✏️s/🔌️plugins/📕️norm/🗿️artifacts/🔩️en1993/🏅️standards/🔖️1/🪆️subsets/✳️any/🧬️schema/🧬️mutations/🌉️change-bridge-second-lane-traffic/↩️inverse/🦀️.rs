//! ↩️ `change-bridge-second-lane-traffic` inverse: restores the previous `second_lane_traffic`.

use crate::change_bridge_second_lane_traffic::ChangeBridgeSecondLaneTraffic;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeSecondLaneTraffic, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_second_lane_traffic::ChangeBridgeSecondLaneTraffic> {
    Ok(match base.bridge_fatigue.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBridgeSecondLaneTraffic { id: payload.id.clone(), new_second_lane_traffic: row.second_lane_traffic })],
        None => Vec::new(),
    })
}
