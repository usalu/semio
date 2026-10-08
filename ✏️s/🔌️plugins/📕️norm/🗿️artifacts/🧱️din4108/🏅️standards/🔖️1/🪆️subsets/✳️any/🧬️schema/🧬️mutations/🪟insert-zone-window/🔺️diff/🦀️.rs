//! 🪟 `insert-zone-window` diff — inserts the window into the zone's list at its position, clamped to the end of the list.

use super::InsertZoneWindow;
use crate::diff::Din4108RowEdit as _;
use crate::diff::{Din4108Diff, Din4108WindowEdit, Din4108ZoneEdit, Din4108ZonePatch};
use crate::Din4108Snapshot;

pub fn diff(payload: &InsertZoneWindow, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    let Some((slot, zone)) = base.zones.iter().enumerate().find(|(_, zone)| zone.id == payload.zone_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "zone not found", Vec::<String>::new());
    };
    let index = payload.index.min(zone.windows.len());
    let nested = vec![Din4108WindowEdit::insert(index, payload.window.clone())];
    protocol::MutationOutcome::new(Din4108Diff { zones: vec![Din4108ZoneEdit::patch(slot, zone.id.clone(), Din4108ZonePatch { windows: nested, ..Default::default() })], ..Default::default() })
}
