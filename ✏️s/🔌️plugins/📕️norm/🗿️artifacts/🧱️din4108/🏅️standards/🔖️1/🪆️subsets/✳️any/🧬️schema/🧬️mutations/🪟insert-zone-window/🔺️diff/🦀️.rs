//! 🪟 `insert-zone-window` diff — inserts the window into the zone's list at its position, clamped to the end of the list.

use super::InsertZoneWindow;
use crate::diff::{Din4108Diff, Din4108WindowDelta, Din4108ZoneDelta, Din4108ZonePatch};
use crate::Din4108Snapshot;

pub fn diff(payload: &InsertZoneWindow, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    let Some(zone) = base.zones.iter().find(|zone| zone.id == payload.zone_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "zone not found", Vec::<String>::new());
    };
    let index = payload.index.min(zone.windows.len());
    let nested = Din4108WindowDelta::insertion(&zone.windows, index, payload.window.clone());
    protocol::MutationOutcome::new(Din4108Diff { zones: Din4108ZoneDelta::modification(&zone.id, Din4108ZonePatch { windows: nested, ..Default::default() }), ..Default::default() })
}
