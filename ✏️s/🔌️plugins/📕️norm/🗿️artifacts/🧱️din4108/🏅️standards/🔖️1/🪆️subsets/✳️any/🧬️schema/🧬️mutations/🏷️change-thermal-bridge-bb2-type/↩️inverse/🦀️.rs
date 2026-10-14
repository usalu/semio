//! ↩️ `change-thermal-bridge-bb2-type` inverse — restores the bridge's `bb2_type`, computed from BASE state; a missing target yields no step.

use crate::change_thermal_bridge_bb2_type::ChangeThermalBridgeBb2Type;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn inverse<Op>(payload: &ChangeThermalBridgeBb2Type, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_thermal_bridge_bb2_type::ChangeThermalBridgeBb2Type> {
    Ok((|| {
    base.thermal_bridges.iter().find(|bridge| bridge.id == payload.bridge_id).map(|bridge| vec![Op::from(ChangeThermalBridgeBb2Type { bridge_id: payload.bridge_id.clone(), new_bb2_type: bridge.bb2_type.clone() })]).unwrap_or_default()

    })())
}
