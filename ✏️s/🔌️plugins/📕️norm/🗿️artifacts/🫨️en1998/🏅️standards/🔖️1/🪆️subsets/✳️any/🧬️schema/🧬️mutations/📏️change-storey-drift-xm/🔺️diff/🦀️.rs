//! Diff for `change-storey-drift-x-m`.
use super::ChangeStoreyDriftXM;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &ChangeStoreyDriftXM, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let mut buildings = base.buildings.clone();
    let Some(b) = buildings.get_mut(payload.building_index) else { return protocol::MutationOutcome::error("mutation.target-missing", "building", Vec::<String>::new()); };
    let Some(st) = b.storeys.get_mut(payload.storey_index) else { return protocol::MutationOutcome::error("mutation.target-missing", "storey", Vec::<String>::new()); };
    st.drift_x_m = payload.new_drift_x_m;
    protocol::MutationOutcome::new(En1998Diff { buildings: Some(buildings), ..Default::default() })
}
