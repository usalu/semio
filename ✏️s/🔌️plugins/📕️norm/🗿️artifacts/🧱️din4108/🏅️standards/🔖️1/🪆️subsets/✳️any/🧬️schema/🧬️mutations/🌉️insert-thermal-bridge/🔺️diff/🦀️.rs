//! 🔺️ `insert-thermal-bridge` diff — inserts the thermal bridge at its position, a whole-list rewrite through the `Din4108Diff` list
//! wrappers; a position past the list's end inserts it last as a `mutation.clamped` warning, and an id the document
//! already holds is a `mutation.duplicate-id`.

use super::InsertThermalBridge;
use crate::standards::v1::subsets::any::schema::diff::{Din4108ElementList, Din4108ThermalBridgeList, Din4108ZoneList};
use crate::{Din4108Diff, Din4108Snapshot};

pub fn diff(payload: &InsertThermalBridge, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if base.thermal_bridges.iter().any(|existing| existing.id == payload.bridge.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A thermal bridge with id '{}' already exists.", payload.bridge.id), [payload.bridge.id.clone()]);
    }
    let mut thermal_bridges = base.thermal_bridges.clone();
    let index = payload.index.min(thermal_bridges.len());
    thermal_bridges.insert(index, payload.bridge.clone());
    let outcome = protocol::MutationOutcome::new(Din4108Diff {
        zones: Some(Din4108ZoneList { values: base.zones.clone() }),
        elements: Some(Din4108ElementList { values: base.elements.clone() }),
        thermal_bridges: Some(Din4108ThermalBridgeList { values: thermal_bridges }),
        ..Default::default()
    });
    if index == payload.index {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the thermal bridge list; inserted at {index}.", payload.index))
}
