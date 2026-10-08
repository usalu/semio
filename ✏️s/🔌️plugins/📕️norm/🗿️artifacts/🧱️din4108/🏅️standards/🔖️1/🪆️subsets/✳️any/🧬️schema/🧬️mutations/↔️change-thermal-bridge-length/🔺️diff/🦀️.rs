//! ↔️ `change-thermal-bridge-length` diff — patches the row's `length_m`; an id the document does not hold is a `mutation.invariant`.

use super::ChangeThermalBridgeLength;
use crate::diff::Din4108RowEdit as _;
use crate::diff::{Din4108Diff, Din4108ThermalBridgeEdit, Din4108ThermalBridgePatch};
use crate::Din4108Snapshot;

pub fn diff(payload: &ChangeThermalBridgeLength, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    let Some((index, row)) = base.thermal_bridges.iter().enumerate().find(|(_, row)| row.id == payload.bridge_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "bridge not found", Vec::<String>::new());
    };
    let patch = Din4108ThermalBridgePatch { length_m: Some(payload.new_length_m), ..Default::default() };
    protocol::MutationOutcome::new(Din4108Diff { thermal_bridges: vec![Din4108ThermalBridgeEdit::patch(index, row.id.clone(), patch)], ..Default::default() })
}
