//! ↩️ `change-thermal-bridge-psi` inverse — restores the bridge's `psi`, computed from BASE state; a missing target yields no step.

use crate::change_thermal_bridge_psi::ChangeThermalBridgePsi;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn inverse<Op>(payload: &ChangeThermalBridgePsi, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_thermal_bridge_psi::ChangeThermalBridgePsi> {
    Ok((|| {
    base.thermal_bridges.iter().find(|bridge| bridge.id == payload.bridge_id).map(|bridge| vec![Op::from(ChangeThermalBridgePsi { bridge_id: payload.bridge_id.clone(), new_psi: bridge.psi })]).unwrap_or_default()

    })())
}
