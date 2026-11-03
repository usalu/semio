//! ➕️ `insert-joint` diff: inserts the joint row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_joint::InsertJoint;
use semio_s_artifact_norm_en1998_diff::{En1998JointDelta, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &InsertJoint, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.joints.iter().any(|existing| existing.id == payload.joint.id) {
        let key = payload.joint.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("joint {key} already exists."), [key]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.joints.len());
    let outcome = protocol::MutationOutcome::new(En1998Diff { joints: En1998JointDelta::insertion(index, payload.joint.clone()), ..Default::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the joint list; inserted at {index}.", payload.index.unwrap_or(index)))
}
