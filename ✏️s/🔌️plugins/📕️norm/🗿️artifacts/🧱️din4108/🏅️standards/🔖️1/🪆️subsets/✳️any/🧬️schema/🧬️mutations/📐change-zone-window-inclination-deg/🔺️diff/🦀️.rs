//! Diff for `change-zone-window-inclination-deg`.

use super::ChangeZoneWindowInclinationDeg;
use crate::standards::v1::subsets::any::schema::diff::{Din4108ElementList, Din4108ThermalBridgeList, Din4108ZoneList};
use crate::{Din4108Diff, Din4108Snapshot};

pub fn diff(payload: &ChangeZoneWindowInclinationDeg, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
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

fn apply_in_place(payload: &ChangeZoneWindowInclinationDeg, snap: &mut Din4108Snapshot) -> Result<(), String> {
    let z = snap.zones.iter_mut().find(|z| z.id == payload.zone_id).ok_or("zone not found")?;
    let w = z.windows.iter_mut().find(|w| w.id == payload.window_id).ok_or("window not found")?;
    w.inclination_deg = payload.new_inclination_deg;
    Ok(())
}
