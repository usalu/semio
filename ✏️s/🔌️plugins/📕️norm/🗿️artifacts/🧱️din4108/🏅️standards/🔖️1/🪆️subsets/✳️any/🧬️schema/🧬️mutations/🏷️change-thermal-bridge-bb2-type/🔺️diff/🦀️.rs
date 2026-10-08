//! 🏷️ `change-thermal-bridge-bb2-type` diff — patches the row's `bb2_type`; an id the document does not hold is a `mutation.invariant`.

use super::ChangeThermalBridgeBb2Type;
use crate::diff::{Din4108Diff, Din4108ThermalBridgeDelta, Din4108ThermalBridgePatch};
use crate::Din4108Snapshot;

pub fn diff(payload: &ChangeThermalBridgeBb2Type, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    let Some(row) = base.thermal_bridges.iter().find(|row| row.id == payload.bridge_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "bridge not found", Vec::<String>::new());
    };
    let patch = Din4108ThermalBridgePatch { bb2_type: Some(payload.new_bb2_type.clone()), ..Default::default() };
    protocol::MutationOutcome::new(Din4108Diff { thermal_bridges: Din4108ThermalBridgeDelta::modification(&row.id, patch), ..Default::default() })
}
