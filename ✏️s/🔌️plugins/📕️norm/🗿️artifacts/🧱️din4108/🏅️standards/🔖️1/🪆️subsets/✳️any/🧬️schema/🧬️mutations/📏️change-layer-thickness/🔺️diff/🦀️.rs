//! 🔺️ `change-layer-thickness` diff — whole-list rewrite via Din4108Diff list wrappers.

use super::ChangeLayerThickness;
use crate::standards::v1::subsets::any::schema::diff::{Din4108ElementList, Din4108ThermalBridgeList, Din4108ZoneList};
use crate::{Din4108Diff, Din4108Snapshot};

pub fn diff(payload: &ChangeLayerThickness, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
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

fn apply_in_place(payload: &ChangeLayerThickness, snap: &mut Din4108Snapshot) -> Result<(), String> {
    
    let e = snap.elements.iter_mut().find(|e| e.id == payload.element_id).ok_or("element not found")?;
    let layer = e.layers.get_mut(payload.index).ok_or("layer index out of range")?;
    layer.thickness_m = payload.new_thickness_m;

    Ok(())
}
