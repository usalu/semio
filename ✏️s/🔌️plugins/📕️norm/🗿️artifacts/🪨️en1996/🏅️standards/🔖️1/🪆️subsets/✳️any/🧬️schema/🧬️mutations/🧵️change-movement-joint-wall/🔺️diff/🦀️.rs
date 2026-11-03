//! 🧵 `change-movement-joint-wall` diff: patches `wall_id` of the addressed movement joint row.

use crate::change_movement_joint_wall::ChangeMovementJointWall;
use semio_s_artifact_norm_en1996_diff::{En1996Diff, En1996JointDelta, En1996JointPatch};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &ChangeMovementJointWall, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    let Some(r0) = base.movement_joints.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Movement joint {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.wall_id == payload.new_wall_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The movement joint already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { movement_joints: En1996JointDelta::modification(&payload.id, En1996JointPatch { wall_id: Some(payload.new_wall_id.clone()), ..Default::default() }), ..Default::default() })
}
