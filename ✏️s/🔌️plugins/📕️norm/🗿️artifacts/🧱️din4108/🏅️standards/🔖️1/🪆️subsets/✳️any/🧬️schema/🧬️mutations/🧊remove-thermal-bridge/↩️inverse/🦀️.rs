//! ↩️ `remove-thermal-bridge` inverse — re-inserts the removed thermal bridge at its position, computed from BASE state; a missing target yields no step.

use crate::remove_thermal_bridge::RemoveThermalBridge;
use crate::insert_thermal_bridge::InsertThermalBridge;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn inverse<Op>(payload: &RemoveThermalBridge, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_thermal_bridge::InsertThermalBridge> {
    Ok((|| {
    base.thermal_bridges.get(payload.index).map(|bridge| vec![Op::from(InsertThermalBridge { index: Some(payload.index), bridge: bridge.clone() })]).unwrap_or_default()

    })())
}
