//! 📏 `change-zone-window-area` diff — patches the window's `area_m2` inside its zone; a missing zone or window is a `mutation.invariant`.

use super::ChangeZoneWindowArea;
use crate::diff::Din4108RowEdit as _;
use crate::diff::{Din4108Diff, Din4108WindowEdit, Din4108WindowPatch, Din4108ZoneEdit, Din4108ZonePatch};
use crate::Din4108Snapshot;

pub fn diff(payload: &ChangeZoneWindowArea, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    let Some((slot, zone)) = base.zones.iter().enumerate().find(|(_, zone)| zone.id == payload.zone_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "zone not found", Vec::<String>::new());
    };
    let Some((at, window)) = zone.windows.iter().enumerate().find(|(_, window)| window.id == payload.window_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "window not found", Vec::<String>::new());
    };
    let nested = vec![Din4108WindowEdit::patch(at, window.id.clone(), Din4108WindowPatch { area_m2: Some(payload.new_area_m2), ..Default::default() })];
    protocol::MutationOutcome::new(Din4108Diff { zones: vec![Din4108ZoneEdit::patch(slot, zone.id.clone(), Din4108ZonePatch { windows: nested, ..Default::default() })], ..Default::default() })
}
