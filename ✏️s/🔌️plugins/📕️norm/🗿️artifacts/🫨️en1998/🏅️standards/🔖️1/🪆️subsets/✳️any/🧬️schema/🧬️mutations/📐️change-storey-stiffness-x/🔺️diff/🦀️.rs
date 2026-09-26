//! Diff for `change-storey-stiffness-x`.
use super::ChangeStoreyStiffnessX;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &ChangeStoreyStiffnessX, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let mut buildings = base.buildings.clone();
    let Some(b) = buildings.get_mut(payload.building_index) else { return protocol::MutationOutcome::error("mutation.target-missing", "building", Vec::<String>::new()); };
    let Some(st) = b.storeys.get_mut(payload.storey_index) else { return protocol::MutationOutcome::error("mutation.target-missing", "storey", Vec::<String>::new()); };
    st.stiffness_x = payload.new_stiffness_x;
    protocol::MutationOutcome::new(En1998Diff { buildings: Some(buildings), ..Default::default() })
}
