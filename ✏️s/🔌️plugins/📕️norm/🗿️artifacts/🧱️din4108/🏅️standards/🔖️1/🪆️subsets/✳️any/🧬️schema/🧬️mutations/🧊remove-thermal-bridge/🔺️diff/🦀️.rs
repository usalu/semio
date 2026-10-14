//! 🧊 `remove-thermal-bridge` diff — removes the row at the index; an index past the list's end is a `mutation.invariant`.

use crate::remove_thermal_bridge::RemoveThermalBridge;
use semio_s_artifact_norm_din4108_diff::{Din4108Diff, Din4108ThermalBridgeDelta};
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn diff(payload: &RemoveThermalBridge, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if payload.index >= base.thermal_bridges.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "bridge index out of range", Vec::<String>::new());
    }
    protocol::MutationOutcome::new(Din4108Diff { thermal_bridges: Din4108ThermalBridgeDelta::removal(&base.thermal_bridges, payload.index), ..Default::default() })
}
