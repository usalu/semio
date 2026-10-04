//! 🔺️ `insert-zone` diff — inserts the zone at its position, a whole-list rewrite through the `Din4108Diff` list
//! wrappers; a position past the list's end inserts it last as a `mutation.clamped` warning, and an id the document
//! already holds is a `mutation.duplicate-id`.

use super::InsertZone;
use crate::standards::v1::subsets::any::schema::diff::{Din4108ElementList, Din4108ThermalBridgeList, Din4108ZoneList};
use crate::{Din4108Diff, Din4108Snapshot};

pub fn diff(payload: &InsertZone, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if base.zones.iter().any(|existing| existing.id == payload.zone.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A zone with id '{}' already exists.", payload.zone.id), [payload.zone.id.clone()]);
    }
    let mut zones = base.zones.clone();
    let index = payload.index.min(zones.len());
    zones.insert(index, payload.zone.clone());
    let outcome = protocol::MutationOutcome::new(Din4108Diff {
        zones: Some(Din4108ZoneList { values: zones }),
        elements: Some(Din4108ElementList { values: base.elements.clone() }),
        thermal_bridges: Some(Din4108ThermalBridgeList { values: base.thermal_bridges.clone() }),
        ..Default::default()
    });
    if index == payload.index {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the zone list; inserted at {index}.", payload.index))
}
