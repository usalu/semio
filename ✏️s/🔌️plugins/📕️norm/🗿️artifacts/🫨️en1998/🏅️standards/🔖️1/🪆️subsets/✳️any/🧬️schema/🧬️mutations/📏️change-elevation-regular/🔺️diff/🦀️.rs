//! 📏️ `change-elevation-regular` diff — patches the one field of the building at the index; a missing building is a `mutation.target-missing`.

use super::ChangeElevationRegular;
use crate::diff::En1998RowEdit as _;
use crate::diff::{En1998BuildingEdit, En1998BuildingPatch, En1998Diff};
use crate::En1998Snapshot;

pub fn diff(payload: &ChangeElevationRegular, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(building) = base.buildings.get(payload.building_index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "building", Vec::<String>::new());
    };
    let patch = En1998BuildingPatch { elevation_regular: Some(payload.new_elevation_regular), ..Default::default() };
    protocol::MutationOutcome::new(En1998Diff { buildings: vec![En1998BuildingEdit::patch(payload.building_index, building.id.clone(), patch)], ..Default::default() })
}
