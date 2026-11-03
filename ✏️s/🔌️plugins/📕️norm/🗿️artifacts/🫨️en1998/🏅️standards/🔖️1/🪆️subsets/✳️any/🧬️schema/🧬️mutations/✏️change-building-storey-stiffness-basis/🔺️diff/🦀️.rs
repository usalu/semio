//! ✏️ `change-building-storey-stiffness-basis` diff: patches `storey_stiffness_basis` of the addressed building row.

use crate::change_building_storey_stiffness_basis::ChangeBuildingStoreyStiffnessBasis;
use semio_s_artifact_norm_en1998_diff::{En1998BuildingDelta, En1998BuildingPatch, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeBuildingStoreyStiffnessBasis, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.buildings.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("building {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.storey_stiffness_basis == payload.new_storey_stiffness_basis {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The building already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { buildings: En1998BuildingDelta::modification(&payload.id, En1998BuildingPatch { storey_stiffness_basis: Some(payload.new_storey_stiffness_basis), ..Default::default() }), ..Default::default() })
}
