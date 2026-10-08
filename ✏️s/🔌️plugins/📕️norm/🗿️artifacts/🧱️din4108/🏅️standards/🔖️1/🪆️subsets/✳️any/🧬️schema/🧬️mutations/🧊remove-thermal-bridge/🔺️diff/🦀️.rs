//! 🧊 `remove-thermal-bridge` diff — removes the row at the index; an index past the list's end is a `mutation.invariant`.

use super::RemoveThermalBridge;
use crate::diff::{Din4108Diff, Din4108ThermalBridgeDelta};
use crate::Din4108Snapshot;

pub fn diff(payload: &RemoveThermalBridge, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    let Some(row) = base.thermal_bridges.get(payload.index) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "bridge index out of range", Vec::<String>::new());
    };
    protocol::MutationOutcome::new(Din4108Diff { thermal_bridges: Din4108ThermalBridgeDelta::removal(&row.id), ..Default::default() })
}
