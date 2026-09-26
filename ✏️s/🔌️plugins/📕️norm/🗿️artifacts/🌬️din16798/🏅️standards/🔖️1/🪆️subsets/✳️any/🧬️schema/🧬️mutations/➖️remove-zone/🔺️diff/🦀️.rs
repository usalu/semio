use super::RemoveZone;
use crate::standards::v1::subsets::any::schema::diff::Din16798ZoneList;
use crate::{Din16798Diff, Din16798Snapshot};
pub fn diff(payload: &RemoveZone, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    let mut next = base.clone();
    next.zones.retain(|z| z.id != payload.zone_id);
    protocol::MutationOutcome::new(Din16798Diff { zones: Some(Din16798ZoneList { values: next.zones }), ..Default::default() })
}
