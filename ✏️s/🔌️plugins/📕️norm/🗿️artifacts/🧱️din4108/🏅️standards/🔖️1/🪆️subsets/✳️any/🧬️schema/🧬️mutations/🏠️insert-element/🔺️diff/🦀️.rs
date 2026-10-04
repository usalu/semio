//! 🔺️ `insert-element` diff — inserts the envelope element at its position, a whole-list rewrite through the `Din4108Diff` list
//! wrappers; a position past the list's end inserts it last as a `mutation.clamped` warning, and an id the document
//! already holds is a `mutation.duplicate-id`.

use super::InsertElement;
use crate::standards::v1::subsets::any::schema::diff::{Din4108ElementList, Din4108ThermalBridgeList, Din4108ZoneList};
use crate::{Din4108Diff, Din4108Snapshot};

pub fn diff(payload: &InsertElement, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if base.elements.iter().any(|existing| existing.id == payload.element.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("An envelope element with id '{}' already exists.", payload.element.id), [payload.element.id.clone()]);
    }
    let mut elements = base.elements.clone();
    let index = payload.index.min(elements.len());
    elements.insert(index, payload.element.clone());
    let outcome = protocol::MutationOutcome::new(Din4108Diff {
        zones: Some(Din4108ZoneList { values: base.zones.clone() }),
        elements: Some(Din4108ElementList { values: elements }),
        thermal_bridges: Some(Din4108ThermalBridgeList { values: base.thermal_bridges.clone() }),
        ..Default::default()
    });
    if index == payload.index {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the envelope element list; inserted at {index}.", payload.index))
}
