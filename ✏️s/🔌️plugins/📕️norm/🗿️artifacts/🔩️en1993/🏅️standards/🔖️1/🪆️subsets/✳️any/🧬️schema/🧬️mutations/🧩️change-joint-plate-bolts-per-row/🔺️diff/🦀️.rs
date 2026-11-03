//! 🔺️ `change-joint-plate-bolts-per-row` diff: patches `bolts_per_row` of the addressed joint plate row.

use crate::change_joint_plate_bolts_per_row::ChangeJointPlateBoltsPerRow;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993JointPlateDelta, En1993JointPlatePatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeJointPlateBoltsPerRow, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.joint_plates.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "joint-plate", Vec::<String>::new());
    };
    if row.bolts_per_row == payload.new_bolts_per_row {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The joint plate already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { joint_plates: En1993JointPlateDelta::modification(payload.id.clone(), En1993JointPlatePatch { bolts_per_row: Some(payload.new_bolts_per_row), ..Default::default() }), ..Default::default() })
}
