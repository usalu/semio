//! 📐️ `change-storey-stiffness-x` diff — patches the one field of the nested row at the indexes; a missing building or storey is a `mutation.target-missing`.

use super::ChangeStoreyStiffnessX;
use crate::diff::{En1998BuildingDelta, En1998BuildingPatch, En1998Diff, En1998StoreyDelta, En1998StoreyPatch};
use crate::En1998Snapshot;

pub fn diff(payload: &ChangeStoreyStiffnessX, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(building) = base.buildings.get(payload.building_index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "building", Vec::<String>::new());
    };
    let Some(row) = building.storeys.get(payload.storey_index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "storey", Vec::<String>::new());
    };
    let nested = En1998StoreyDelta::modification(&row.id, En1998StoreyPatch { stiffness_x: Some(payload.new_stiffness_x), ..Default::default() });
    let patch = En1998BuildingPatch { storeys: nested, ..Default::default() };
    protocol::MutationOutcome::new(En1998Diff { buildings: En1998BuildingDelta::modification(&building.id, patch), ..Default::default() })
}
