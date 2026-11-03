//! 🔺️ `change-joint-bolts-per-row` diff: patches `bolts_per_row` of the addressed joint row.

use crate::change_joint_bolts_per_row::ChangeJointBoltsPerRow;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993JointDelta, En1993JointPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeJointBoltsPerRow, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.joints.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "joint", Vec::<String>::new());
    };
    if row.bolts_per_row == payload.new_bolts_per_row {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The joint already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { joints: En1993JointDelta::modification(payload.id.clone(), En1993JointPatch { bolts_per_row: Some(payload.new_bolts_per_row), ..Default::default() }), ..Default::default() })
}
