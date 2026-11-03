//! ✏️ `change-building-joint-gap-m` diff: patches `joint_gap_m` of the addressed building row.

use crate::change_building_joint_gap_m::ChangeBuildingJointGapM;
use semio_s_artifact_norm_en1998_diff::{En1998BuildingDelta, En1998BuildingPatch, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeBuildingJointGapM, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_joint_gap_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.buildings.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("building {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.joint_gap_m == payload.new_joint_gap_m {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The building already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { buildings: En1998BuildingDelta::modification(&payload.id, En1998BuildingPatch { joint_gap_m: Some(payload.new_joint_gap_m), ..Default::default() }), ..Default::default() })
}
