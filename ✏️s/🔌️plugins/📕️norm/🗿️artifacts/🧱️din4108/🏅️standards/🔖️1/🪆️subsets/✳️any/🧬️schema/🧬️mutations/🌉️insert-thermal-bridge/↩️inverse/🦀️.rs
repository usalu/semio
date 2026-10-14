//! ↩️ `insert-thermal-bridge` inverse — removes the inserted thermal bridge at its landing position, computed from BASE state; a missing target yields no step.

use crate::insert_thermal_bridge::InsertThermalBridge;
use crate::remove_thermal_bridge::RemoveThermalBridge;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn inverse<Op>(payload: &InsertThermalBridge, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_thermal_bridge::RemoveThermalBridge> {
    Ok((|| {
    vec![Op::from(RemoveThermalBridge { index: payload.index.unwrap_or(usize::MAX).min(base.thermal_bridges.len()) })]

    })())
}
