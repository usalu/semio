//! 🔺️ `change-vent-sfp` diff.
use super::ChangeVentSfp;
use crate::standards::v1::subsets::any::schema::diff::Din16798VentList;
use crate::{Din16798Diff, Din16798Snapshot};
pub fn diff(payload: &ChangeVentSfp, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    let mut next = base.clone();
    let Some(v) = next.vent_systems.iter_mut().find(|v| v.id == payload.vent_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "vent not found", Vec::<String>::new());
    };
    v.sfp_w_m3_s = payload.new_sfp_w_m3_s;
    protocol::MutationOutcome::new(Din16798Diff { vent_systems: Some(Din16798VentList { values: next.vent_systems }), ..Default::default() })
}
