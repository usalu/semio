//! 🔺️ `change-joint-bolt-class` diff: patches `bolt_class` of the addressed joint row.

use crate::change_joint_bolt_class::ChangeJointBoltClass;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993JointDelta, En1993JointPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeJointBoltClass, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.joints.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "joint", Vec::<String>::new());
    };
    if row.bolt_class == payload.new_bolt_class {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The joint already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { joints: En1993JointDelta::modification(payload.id.clone(), En1993JointPatch { bolt_class: Some(payload.new_bolt_class.clone()), ..Default::default() }), ..Default::default() })
}
