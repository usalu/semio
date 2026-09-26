use super::InsertVentSystem;
use crate::standards::v1::subsets::any::schema::diff::Din16798VentList;
use crate::{Din16798Diff, Din16798Snapshot};
pub fn diff(payload: &InsertVentSystem, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    let mut next = base.clone();
    let idx = payload.index.min(next.vent_systems.len());
    next.vent_systems.insert(idx, payload.vent.clone());
    protocol::MutationOutcome::new(Din16798Diff { vent_systems: Some(Din16798VentList { values: next.vent_systems }), ..Default::default() })
}
