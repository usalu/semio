//! Diff for `change-thermal-bridge-bb2-type`.

use super::ChangeThermalBridgeBb2Type;
use crate::standards::v1::subsets::any::schema::diff::{Din4108ElementList, Din4108ThermalBridgeList, Din4108ZoneList};
use crate::{Din4108Diff, Din4108Snapshot};

pub fn diff(payload: &ChangeThermalBridgeBb2Type, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
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

fn apply_in_place(payload: &ChangeThermalBridgeBb2Type, snap: &mut Din4108Snapshot) -> Result<(), String> {
    let b = snap.thermal_bridges.iter_mut().find(|b| b.id == payload.bridge_id).ok_or("bridge not found")?;
    b.bb2_type = payload.new_bb2_type.clone();
    Ok(())
}
