//! 🚫️ `remove-zone-window` diff — removes the window at the index from the zone's list.

use super::RemoveZoneWindow;
use crate::diff::{Din4108Diff, Din4108WindowDelta, Din4108ZoneDelta, Din4108ZonePatch};
use crate::Din4108Snapshot;

pub fn diff(payload: &RemoveZoneWindow, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    let Some(zone) = base.zones.iter().find(|zone| zone.id == payload.zone_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "zone not found", Vec::<String>::new());
    };
    if payload.index >= zone.windows.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "window index out of range", Vec::<String>::new());
    }
    let nested = Din4108WindowDelta::removal(&zone.windows, payload.index);
    protocol::MutationOutcome::new(Din4108Diff { zones: Din4108ZoneDelta::modification(&zone.id, Din4108ZonePatch { windows: nested, ..Default::default() }), ..Default::default() })
}
