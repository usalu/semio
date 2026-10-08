//! ☀️ `change-zone-window-g-value` diff — patches the window's `g_value` inside its zone; a missing zone or window is a `mutation.invariant`.

use super::ChangeZoneWindowGValue;
use crate::diff::{Din4108Diff, Din4108WindowDelta, Din4108WindowPatch, Din4108ZoneDelta, Din4108ZonePatch};
use crate::Din4108Snapshot;

pub fn diff(payload: &ChangeZoneWindowGValue, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    let Some(zone) = base.zones.iter().find(|zone| zone.id == payload.zone_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "zone not found", Vec::<String>::new());
    };
    let Some(window) = zone.windows.iter().find(|window| window.id == payload.window_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "window not found", Vec::<String>::new());
    };
    let nested = Din4108WindowDelta::modification(&window.id, Din4108WindowPatch { g_value: Some(payload.new_g_value), ..Default::default() });
    protocol::MutationOutcome::new(Din4108Diff { zones: Din4108ZoneDelta::modification(&zone.id, Din4108ZonePatch { windows: nested, ..Default::default() }), ..Default::default() })
}
