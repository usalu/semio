//! ↩️ `change-thermal-bridge-length` inverse — restores the bridge's `length_m`, computed from BASE state; a missing target yields no step.

use super::ChangeThermalBridgeLength;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(payload: &ChangeThermalBridgeLength, base: &Din4108Snapshot) -> Result<Vec<Din4108Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    base.thermal_bridges.iter().find(|bridge| bridge.id == payload.bridge_id).map(|bridge| vec![Din4108Mutation::ChangeThermalBridgeLength(ChangeThermalBridgeLength { bridge_id: payload.bridge_id.clone(), new_length_m: bridge.length_m })]).unwrap_or_default()

    })())
}
