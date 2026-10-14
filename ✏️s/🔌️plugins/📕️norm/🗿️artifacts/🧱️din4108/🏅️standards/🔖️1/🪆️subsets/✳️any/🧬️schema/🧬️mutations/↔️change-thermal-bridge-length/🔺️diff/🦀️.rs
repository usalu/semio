//! ↔️ `change-thermal-bridge-length` diff — patches the row's `length_m`; an id the document does not hold is a `mutation.invariant`.

use crate::change_thermal_bridge_length::ChangeThermalBridgeLength;
use semio_s_artifact_norm_din4108_diff::{Din4108Diff, Din4108ThermalBridgeDelta, Din4108ThermalBridgePatch};
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn diff(payload: &ChangeThermalBridgeLength, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    let Some(row) = base.thermal_bridges.iter().find(|row| row.id == payload.bridge_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "bridge not found", Vec::<String>::new());
    };
    let patch = Din4108ThermalBridgePatch { length_m: Some(payload.new_length_m), ..Default::default() };
    protocol::MutationOutcome::new(Din4108Diff { thermal_bridges: Din4108ThermalBridgeDelta::modification(&row.id, patch), ..Default::default() })
}
