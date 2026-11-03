//! 🔺️ `change-joint-column-root-radius` diff: patches `column_root_radius` of the addressed joint row.

use crate::change_joint_column_root_radius::ChangeJointColumnRootRadius;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993JointDelta, En1993JointPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeJointColumnRootRadius, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_column_root_radius.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.joints.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "joint", Vec::<String>::new());
    };
    if row.column_root_radius == payload.new_column_root_radius {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The joint already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { joints: En1993JointDelta::modification(payload.id.clone(), En1993JointPatch { column_root_radius: Some(payload.new_column_root_radius), ..Default::default() }), ..Default::default() })
}
