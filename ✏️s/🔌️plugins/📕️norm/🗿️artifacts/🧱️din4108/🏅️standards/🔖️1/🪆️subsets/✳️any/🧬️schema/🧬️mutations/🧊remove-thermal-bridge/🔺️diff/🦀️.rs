//! 🧊 `remove-thermal-bridge` diff — removes the row at the index, guarded by the row's own id; an index past the list's end is a `mutation.invariant`.

use super::RemoveThermalBridge;
use crate::diff::Din4108RowEdit as _;
use crate::diff::{Din4108Diff, Din4108ThermalBridgeEdit};
use crate::Din4108Snapshot;

pub fn diff(payload: &RemoveThermalBridge, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    let Some(row) = base.thermal_bridges.get(payload.index) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "bridge index out of range", Vec::<String>::new());
    };
    protocol::MutationOutcome::new(Din4108Diff { thermal_bridges: vec![Din4108ThermalBridgeEdit::remove(payload.index, row.id.clone())], ..Default::default() })
}
