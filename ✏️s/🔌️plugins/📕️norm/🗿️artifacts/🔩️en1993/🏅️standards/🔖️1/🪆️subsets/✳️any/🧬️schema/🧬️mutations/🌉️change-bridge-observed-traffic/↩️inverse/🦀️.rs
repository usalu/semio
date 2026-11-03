//! ↩️ `change-bridge-observed-traffic` inverse: restores the previous `observed_traffic`.

use crate::change_bridge_observed_traffic::ChangeBridgeObservedTraffic;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeObservedTraffic, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_observed_traffic::ChangeBridgeObservedTraffic> {
    Ok(match base.bridge_fatigue.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBridgeObservedTraffic { id: payload.id.clone(), new_observed_traffic: row.observed_traffic })],
        None => Vec::new(),
    })
}
