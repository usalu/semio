//! ➖️ `remove-fastened-joint` diff — removes the row at the index.

use crate::remove_fastened_joint::RemoveFastenedJoint;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FastenedJointDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &RemoveFastenedJoint, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.fastened_joints.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("fastened-joint index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { fastened_joints: En1993FastenedJointDelta::removal(&base.fastened_joints, payload.index), ..Default::default() })
}
