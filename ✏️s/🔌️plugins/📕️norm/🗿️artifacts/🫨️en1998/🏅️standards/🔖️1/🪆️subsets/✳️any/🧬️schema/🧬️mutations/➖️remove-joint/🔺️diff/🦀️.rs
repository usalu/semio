//! ➖️ `remove-joint` diff: removes the addressed joint row.

use crate::remove_joint::RemoveJoint;
use semio_s_artifact_norm_en1998_diff::{En1998JointDelta, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &RemoveJoint, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(index) = base.joints.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("joint {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1998Diff { joints: En1998JointDelta::removal(&base.joints, index), ..Default::default() })
}
