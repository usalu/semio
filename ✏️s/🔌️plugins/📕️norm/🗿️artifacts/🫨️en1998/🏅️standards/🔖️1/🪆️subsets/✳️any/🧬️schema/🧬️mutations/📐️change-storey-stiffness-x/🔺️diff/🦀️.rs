//! 📐️ `change-storey-stiffness-x` diff: patches `stiffness_x` of the addressed storey row.

use crate::change_storey_stiffness_x::ChangeStoreyStiffnessX;
use semio_s_artifact_norm_en1998_diff::{En1998BuildingDelta, En1998BuildingPatch, En1998Diff, En1998StoreyDelta, En1998StoreyPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeStoreyStiffnessX, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_stiffness_x.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.buildings.iter().find(|row| row.id == payload.building_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("building {} does not exist.", payload.building_id), [payload.building_id.clone()]);
    };
    let Some(r1) = r0.storeys.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("storey {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r1.stiffness_x == payload.new_stiffness_x {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The storey already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { buildings: En1998BuildingDelta::modification(&payload.building_id, En1998BuildingPatch { storeys: En1998StoreyDelta::modification(&payload.id, En1998StoreyPatch { stiffness_x: Some(payload.new_stiffness_x), ..Default::default() }), ..Default::default() }), ..Default::default() })
}
