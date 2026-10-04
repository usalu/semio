//! ↩️ `insert-thermal-bridge` inverse — removes the inserted thermal bridge at its landing position, computed from BASE state; a missing target yields no step.

use super::InsertThermalBridge;
use crate::mutations::remove_thermal_bridge::RemoveThermalBridge;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(payload: &InsertThermalBridge, base: &Din4108Snapshot) -> Result<Vec<Din4108Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Din4108Mutation::RemoveThermalBridge(RemoveThermalBridge { index: payload.index.min(base.thermal_bridges.len()) })]

    })())
}
