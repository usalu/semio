//! ➖️ `remove-joint` diff — removes the row at the index.

use super::RemoveJoint;
use crate::diff::{En1993Diff, En1993JointDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &RemoveJoint, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.joints.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("joint index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { joints: En1993JointDelta::removal(&base.joints, payload.index), ..Default::default() })
}
