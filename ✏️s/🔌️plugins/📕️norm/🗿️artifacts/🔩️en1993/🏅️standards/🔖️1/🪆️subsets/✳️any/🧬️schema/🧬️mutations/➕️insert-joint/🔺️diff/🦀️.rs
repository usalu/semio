use super::InsertJoint;
use crate::diff::En1993JointList;
use crate::{En1993Diff, En1993Snapshot};
pub fn diff(payload: &InsertJoint, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let mut values = base.joints.clone();
    let at = payload.index.min(values.len());
    values.insert(at, payload.joint.clone());
    protocol::MutationOutcome::new(En1993Diff { joints: Some(En1993JointList { values }), ..Default::default() })
}
