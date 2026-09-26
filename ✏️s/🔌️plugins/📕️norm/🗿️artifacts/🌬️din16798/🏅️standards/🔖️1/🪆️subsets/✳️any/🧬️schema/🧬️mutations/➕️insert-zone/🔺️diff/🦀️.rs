use super::InsertZone;
use crate::standards::v1::subsets::any::schema::diff::Din16798ZoneList;
use crate::{Din16798Diff, Din16798Snapshot};
pub fn diff(payload: &InsertZone, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    let mut next = base.clone();
    let idx = payload.index.min(next.zones.len());
    next.zones.insert(idx, payload.zone.clone());
    protocol::MutationOutcome::new(Din16798Diff { zones: Some(Din16798ZoneList { values: next.zones }), ..Default::default() })
}
