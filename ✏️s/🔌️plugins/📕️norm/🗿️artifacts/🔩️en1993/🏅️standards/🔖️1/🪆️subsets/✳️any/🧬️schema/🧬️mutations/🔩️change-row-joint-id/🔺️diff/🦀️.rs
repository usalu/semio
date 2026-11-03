//! 🔺️ `change-row-joint-id` diff: patches `joint_id` of the addressed joint row row.

use crate::change_row_joint_id::ChangeRowJointId;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993JointRowDelta, En1993JointRowPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeRowJointId, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.joint_rows.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "joint-row", Vec::<String>::new());
    };
    if row.joint_id == payload.new_joint_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The joint row already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { joint_rows: En1993JointRowDelta::modification(payload.id.clone(), En1993JointRowPatch { joint_id: Some(payload.new_joint_id.clone()), ..Default::default() }), ..Default::default() })
}
