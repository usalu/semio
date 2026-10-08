//! 🌉️ `insert-thermal-bridge` diff — inserts the row at its position; a position past the list's end inserts it last as a
//! `mutation.clamped` warning, and an id the document already holds is a `mutation.duplicate-id`.

use super::InsertThermalBridge;
use crate::diff::{Din4108Diff, Din4108ThermalBridgeDelta};
use crate::Din4108Snapshot;

pub fn diff(payload: &InsertThermalBridge, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if base.thermal_bridges.iter().any(|existing| existing.id == payload.bridge.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A thermal bridge with id '{}' already exists.", payload.bridge.id), [payload.bridge.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.thermal_bridges.len());
    let outcome = protocol::MutationOutcome::new(Din4108Diff { thermal_bridges: Din4108ThermalBridgeDelta::insertion(index, payload.bridge.clone()), ..Default::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the thermal bridge list; inserted at {index}.", payload.index.unwrap_or(index)))
}
