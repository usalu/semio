//! ↩️ `remove-thermal-bridge` inverse — re-inserts the removed thermal bridge at its position, computed from BASE state; a missing target yields no step.

use super::RemoveThermalBridge;
use crate::mutations::insert_thermal_bridge::InsertThermalBridge;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(payload: &RemoveThermalBridge, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    base.thermal_bridges.get(payload.index).map(|bridge| vec![Din4108Mutation::InsertThermalBridge(InsertThermalBridge { index: payload.index, bridge: bridge.clone() })]).unwrap_or_default()
}
