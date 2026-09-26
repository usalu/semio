use super::RemoveVentSystem;
use crate::standards::v1::subsets::any::schema::diff::Din16798VentList;
use crate::{Din16798Diff, Din16798Snapshot};
pub fn diff(payload: &RemoveVentSystem, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    let mut next = base.clone();
    next.vent_systems.retain(|v| v.id != payload.vent_id);
    protocol::MutationOutcome::new(Din16798Diff { vent_systems: Some(Din16798VentList { values: next.vent_systems }), ..Default::default() })
}
