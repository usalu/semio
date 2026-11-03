//! ↩️ `change-bridge-heavy-vehicle-weight` inverse: restores the previous `heavy_vehicle_weight`.

use crate::change_bridge_heavy_vehicle_weight::ChangeBridgeHeavyVehicleWeight;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeHeavyVehicleWeight, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_heavy_vehicle_weight::ChangeBridgeHeavyVehicleWeight> {
    Ok(match base.bridge_fatigue.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBridgeHeavyVehicleWeight { id: payload.id.clone(), new_heavy_vehicle_weight: row.heavy_vehicle_weight })],
        None => Vec::new(),
    })
}
