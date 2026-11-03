//! ➖️ `remove-hollow-joint` diff — removes the row at the index.

use crate::remove_hollow_joint::RemoveHollowJoint;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993HollowJointDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &RemoveHollowJoint, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.hollow_joints.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("hollow-joint index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { hollow_joints: En1993HollowJointDelta::removal(&base.hollow_joints, payload.index), ..Default::default() })
}
