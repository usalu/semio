//! ↩️ `change-thermal-bridge-length` inverse — restores the bridge's `length_m`, computed from BASE state; a missing target yields no step.

use crate::change_thermal_bridge_length::ChangeThermalBridgeLength;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn inverse<Op>(payload: &ChangeThermalBridgeLength, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_thermal_bridge_length::ChangeThermalBridgeLength> {
    Ok((|| {
    base.thermal_bridges.iter().find(|bridge| bridge.id == payload.bridge_id).map(|bridge| vec![Op::from(ChangeThermalBridgeLength { bridge_id: payload.bridge_id.clone(), new_length_m: bridge.length_m })]).unwrap_or_default()

    })())
}
