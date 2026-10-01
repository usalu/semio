//! 🔺️ `remove-zone` diff — removes the zone with that id; an id the document does not hold is a `mutation.target-missing`.

use super::RemoveZone;
use crate::standards::v1::subsets::any::schema::diff::Din16798ZoneList;
use crate::{Din16798Diff, Din16798Snapshot};

pub fn diff(payload: &RemoveZone, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !base.zones.iter().any(|zone| zone.id == payload.zone_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No zone has id '{}'.", payload.zone_id), [payload.zone_id.clone()]);
    }
    let zones = base.zones.iter().filter(|zone| zone.id != payload.zone_id).cloned().collect();
    protocol::MutationOutcome::new(Din16798Diff { zones: Some(Din16798ZoneList { values: zones }), ..Default::default() })
}
