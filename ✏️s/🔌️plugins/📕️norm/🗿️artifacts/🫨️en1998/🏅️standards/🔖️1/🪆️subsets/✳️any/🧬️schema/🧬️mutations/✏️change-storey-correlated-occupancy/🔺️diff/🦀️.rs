//! ✏️ `change-storey-correlated-occupancy` diff: patches `correlated_occupancy` of the addressed storey row.

use crate::change_storey_correlated_occupancy::ChangeStoreyCorrelatedOccupancy;
use semio_s_artifact_norm_en1998_diff::{En1998BuildingDelta, En1998BuildingPatch, En1998Diff, En1998StoreyDelta, En1998StoreyPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeStoreyCorrelatedOccupancy, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.buildings.iter().find(|row| row.id == payload.building_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("building {} does not exist.", payload.building_id), [payload.building_id.clone()]);
    };
    let Some(r1) = r0.storeys.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("storey {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r1.correlated_occupancy == payload.new_correlated_occupancy {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The storey already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { buildings: En1998BuildingDelta::modification(&payload.building_id, En1998BuildingPatch { storeys: En1998StoreyDelta::modification(&payload.id, En1998StoreyPatch { correlated_occupancy: Some(payload.new_correlated_occupancy), ..Default::default() }), ..Default::default() }), ..Default::default() })
}
