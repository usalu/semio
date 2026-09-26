//! 🔺️ `reorder-layers` diff — whole-list rewrite via Din4108Diff list wrappers.

use super::ReorderLayers;
use crate::standards::v1::subsets::any::schema::diff::{Din4108ElementList, Din4108ThermalBridgeList, Din4108ZoneList};
use crate::{Din4108Diff, Din4108Snapshot};

pub fn diff(payload: &ReorderLayers, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    let mut next = base.clone();
    if let Err(msg) = apply_in_place(payload, &mut next) {
        return protocol::MutationOutcome::fatal("mutation.invariant", msg, Vec::<String>::new());
    }
    protocol::MutationOutcome::new(Din4108Diff {
        zones: Some(Din4108ZoneList { values: next.zones }),
        elements: Some(Din4108ElementList { values: next.elements }),
        thermal_bridges: Some(Din4108ThermalBridgeList { values: next.thermal_bridges }),
        ..Default::default()
    })
}

fn apply_in_place(payload: &ReorderLayers, snap: &mut Din4108Snapshot) -> Result<(), String> {
    
    let e = snap.elements.iter_mut().find(|e| e.id == payload.element_id).ok_or("element not found")?;
    if payload.from >= e.layers.len() || payload.to >= e.layers.len() { return Err("layer reorder out of range".into()); }
    let layer = e.layers.remove(payload.from);
    e.layers.insert(payload.to, layer);

    Ok(())
}
