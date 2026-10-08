//! 🧱 `change-zone-heaviness` diff — patches the row's `heaviness`; an id the document does not hold is a `mutation.invariant`.

use super::ChangeZoneHeaviness;
use crate::diff::{Din4108Diff, Din4108ZoneDelta, Din4108ZonePatch};
use crate::Din4108Snapshot;

pub fn diff(payload: &ChangeZoneHeaviness, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    let Some(row) = base.zones.iter().find(|row| row.id == payload.zone_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "zone not found", Vec::<String>::new());
    };
    let patch = Din4108ZonePatch { heaviness: Some(payload.new_heaviness.clone()), ..Default::default() };
    protocol::MutationOutcome::new(Din4108Diff { zones: Din4108ZoneDelta::modification(&row.id, patch), ..Default::default() })
}
