//! Diff for `change-building-elevation-regular`.
use super::ChangeBuildingElevationRegular;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &ChangeBuildingElevationRegular, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let mut buildings = base.buildings.clone();
    let Some(b) = buildings.get_mut(payload.building_index) else { return protocol::MutationOutcome::error("mutation.target-missing", "building", Vec::<String>::new()); };
    b.elevation_regular = payload.new_elevation_regular;
    protocol::MutationOutcome::new(En1998Diff { buildings: Some(buildings), ..Default::default() })
}
