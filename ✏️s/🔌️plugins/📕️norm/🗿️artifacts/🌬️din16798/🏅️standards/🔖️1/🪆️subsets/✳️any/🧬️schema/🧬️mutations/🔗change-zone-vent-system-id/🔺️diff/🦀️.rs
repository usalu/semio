//! 🔺️ `change-zone-vent-system-id` diff.
use super::ChangeZoneVentSystemId;
use crate::standards::v1::subsets::any::schema::diff::Din16798ZoneList;
use crate::{Din16798Diff, Din16798Snapshot};
pub fn diff(payload: &ChangeZoneVentSystemId, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    let mut next = base.clone();
    let Some(z) = next.zones.iter_mut().find(|z| z.id == payload.zone_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "zone not found", Vec::<String>::new());
    };
    z.vent_system_id = payload.new_vent_system_id.clone();
    protocol::MutationOutcome::new(Din16798Diff { zones: Some(Din16798ZoneList { values: next.zones }), ..Default::default() })
}
