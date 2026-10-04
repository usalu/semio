//! ↩️ `change-thermal-bridge-bb2-type` inverse — restores the bridge's `bb2_type`, computed from BASE state; a missing target yields no step.

use super::ChangeThermalBridgeBb2Type;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(payload: &ChangeThermalBridgeBb2Type, base: &Din4108Snapshot) -> Result<Vec<Din4108Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    base.thermal_bridges.iter().find(|bridge| bridge.id == payload.bridge_id).map(|bridge| vec![Din4108Mutation::ChangeThermalBridgeBb2Type(ChangeThermalBridgeBb2Type { bridge_id: payload.bridge_id.clone(), new_bb2_type: bridge.bb2_type.clone() })]).unwrap_or_default()

    })())
}
