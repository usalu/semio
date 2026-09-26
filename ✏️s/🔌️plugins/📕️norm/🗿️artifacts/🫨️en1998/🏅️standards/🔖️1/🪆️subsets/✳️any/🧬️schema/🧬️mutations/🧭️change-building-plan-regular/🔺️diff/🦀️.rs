//! Diff for `change-building-plan-regular`.
use super::ChangeBuildingPlanRegular;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &ChangeBuildingPlanRegular, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let mut buildings = base.buildings.clone();
    let Some(b) = buildings.get_mut(payload.building_index) else { return protocol::MutationOutcome::error("mutation.target-missing", "building", Vec::<String>::new()); };
    b.plan_regular = payload.new_plan_regular;
    protocol::MutationOutcome::new(En1998Diff { buildings: Some(buildings), ..Default::default() })
}
