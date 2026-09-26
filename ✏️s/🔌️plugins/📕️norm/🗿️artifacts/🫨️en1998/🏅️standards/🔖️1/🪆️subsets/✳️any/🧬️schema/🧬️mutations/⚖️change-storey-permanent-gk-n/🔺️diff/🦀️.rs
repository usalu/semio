//! Diff for `change-storey-permanent-gk-n`.
use super::ChangeStoreyPermanentGkN;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &ChangeStoreyPermanentGkN, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let mut buildings = base.buildings.clone();
    let Some(b) = buildings.get_mut(payload.building_index) else { return protocol::MutationOutcome::error("mutation.target-missing", "building", Vec::<String>::new()); };
    let Some(st) = b.storeys.get_mut(payload.storey_index) else { return protocol::MutationOutcome::error("mutation.target-missing", "storey", Vec::<String>::new()); };
    st.permanent_gk_n = payload.new_permanent_gk_n;
    protocol::MutationOutcome::new(En1998Diff { buildings: Some(buildings), ..Default::default() })
}
