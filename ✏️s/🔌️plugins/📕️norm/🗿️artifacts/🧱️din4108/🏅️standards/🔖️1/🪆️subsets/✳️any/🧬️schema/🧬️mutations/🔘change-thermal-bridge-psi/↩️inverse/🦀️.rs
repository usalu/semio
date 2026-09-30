//! ↩️ `change-thermal-bridge-psi` inverse — restores the bridge's `psi`, computed from BASE state; a missing target yields no step.

use super::ChangeThermalBridgePsi;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(payload: &ChangeThermalBridgePsi, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    base.thermal_bridges.iter().find(|bridge| bridge.id == payload.bridge_id).map(|bridge| vec![Din4108Mutation::ChangeThermalBridgePsi(ChangeThermalBridgePsi { bridge_id: payload.bridge_id.clone(), new_psi: bridge.psi })]).unwrap_or_default()
}
