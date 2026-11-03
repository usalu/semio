//! 🔺️ `change-joint-plate-edge-distance` diff: patches `edge_distance` of the addressed joint plate row.

use crate::change_joint_plate_edge_distance::ChangeJointPlateEdgeDistance;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993JointPlateDelta, En1993JointPlatePatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeJointPlateEdgeDistance, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_edge_distance.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.joint_plates.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "joint-plate", Vec::<String>::new());
    };
    if row.edge_distance == payload.new_edge_distance {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The joint plate already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { joint_plates: En1993JointPlateDelta::modification(payload.id.clone(), En1993JointPlatePatch { edge_distance: Some(payload.new_edge_distance), ..Default::default() }), ..Default::default() })
}
