//! 🔘 `change-thermal-bridge-psi` diff — patches the row's `psi`; an id the document does not hold is a `mutation.invariant`.

use super::ChangeThermalBridgePsi;
use crate::diff::Din4108RowEdit as _;
use crate::diff::{Din4108Diff, Din4108ThermalBridgeEdit, Din4108ThermalBridgePatch};
use crate::Din4108Snapshot;

pub fn diff(payload: &ChangeThermalBridgePsi, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    let Some((index, row)) = base.thermal_bridges.iter().enumerate().find(|(_, row)| row.id == payload.bridge_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "bridge not found", Vec::<String>::new());
    };
    let patch = Din4108ThermalBridgePatch { psi: Some(payload.new_psi), ..Default::default() };
    protocol::MutationOutcome::new(Din4108Diff { thermal_bridges: vec![Din4108ThermalBridgeEdit::patch(index, row.id.clone(), patch)], ..Default::default() })
}
