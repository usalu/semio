//! 🚫️ `remove-zone-window` diff — removes the window at the index from the zone's list, guarded by the window's own id.

use super::RemoveZoneWindow;
use crate::diff::Din4108RowEdit as _;
use crate::diff::{Din4108Diff, Din4108WindowEdit, Din4108ZoneEdit, Din4108ZonePatch};
use crate::Din4108Snapshot;

pub fn diff(payload: &RemoveZoneWindow, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    let Some((slot, zone)) = base.zones.iter().enumerate().find(|(_, zone)| zone.id == payload.zone_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "zone not found", Vec::<String>::new());
    };
    let Some(window) = zone.windows.get(payload.index) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "window index out of range", Vec::<String>::new());
    };
    let nested = vec![Din4108WindowEdit::remove(payload.index, window.id.clone())];
    protocol::MutationOutcome::new(Din4108Diff { zones: vec![Din4108ZoneEdit::patch(slot, zone.id.clone(), Din4108ZonePatch { windows: nested, ..Default::default() })], ..Default::default() })
}
