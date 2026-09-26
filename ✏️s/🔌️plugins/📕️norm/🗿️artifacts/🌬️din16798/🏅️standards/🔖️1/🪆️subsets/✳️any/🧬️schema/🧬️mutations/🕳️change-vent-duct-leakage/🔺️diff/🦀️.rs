//! 🔺️ `change-vent-duct-leakage` diff.
use super::ChangeVentDuctLeakage;
use crate::standards::v1::subsets::any::schema::diff::Din16798VentList;
use crate::{Din16798Diff, Din16798Snapshot};
pub fn diff(payload: &ChangeVentDuctLeakage, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    let mut next = base.clone();
    let Some(v) = next.vent_systems.iter_mut().find(|v| v.id == payload.vent_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "vent not found", Vec::<String>::new());
    };
    v.duct_leakage_m3_s_m2 = payload.new_duct_leakage_m3_s_m2;
    protocol::MutationOutcome::new(Din16798Diff { vent_systems: Some(Din16798VentList { values: next.vent_systems }), ..Default::default() })
}
