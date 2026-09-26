//! 🔺️ `change-vent-design-airflow` diff.
use super::ChangeVentDesignAirflow;
use crate::standards::v1::subsets::any::schema::diff::Din16798VentList;
use crate::{Din16798Diff, Din16798Snapshot};
pub fn diff(payload: &ChangeVentDesignAirflow, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    let mut next = base.clone();
    let Some(item) = next.vent_systems.iter_mut().find(|x| x.id == payload.vent_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "vent not found", Vec::<String>::new());
    };
    item.design_airflow_m3_h = payload.new_design_airflow_m3_h;
    protocol::MutationOutcome::new(Din16798Diff { vent_systems: Some(Din16798VentList { values: next.vent_systems }), ..Default::default() })
}
