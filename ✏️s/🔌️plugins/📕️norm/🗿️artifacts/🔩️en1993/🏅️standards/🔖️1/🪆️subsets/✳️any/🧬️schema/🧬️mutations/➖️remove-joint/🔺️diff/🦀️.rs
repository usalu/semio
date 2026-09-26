use super::RemoveJoint;
use crate::diff::En1993JointList;
use crate::{En1993Diff, En1993Snapshot};
pub fn diff(payload: &RemoveJoint, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.joints.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("joint index {} out of range.", payload.index), Vec::<String>::new());
    }
    let mut values = base.joints.clone();
    values.remove(payload.index);
    protocol::MutationOutcome::new(En1993Diff { joints: Some(En1993JointList { values }), ..Default::default() })
}
