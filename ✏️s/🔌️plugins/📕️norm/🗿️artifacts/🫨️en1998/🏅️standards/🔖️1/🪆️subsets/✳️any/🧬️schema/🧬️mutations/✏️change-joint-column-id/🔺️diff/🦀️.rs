//! ✏️ `change-joint-column-id` diff: patches `column_id` of the addressed joint row.

use crate::change_joint_column_id::ChangeJointColumnId;
use semio_s_artifact_norm_en1998_diff::{En1998JointDelta, En1998JointPatch, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeJointColumnId, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.joints.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("joint {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.column_id == payload.new_column_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The joint already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { joints: En1998JointDelta::modification(&payload.id, En1998JointPatch { column_id: Some(payload.new_column_id.clone()), ..Default::default() }), ..Default::default() })
}
