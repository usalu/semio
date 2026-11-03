//! 🔺️ `change-fastened-joint-base-strength` diff: patches `base_strength` of the addressed fastened_joints block row.

use crate::change_fastened_joint_base_strength::ChangeFastenedJointBaseStrength;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FastenedJointsDelta, En1993FastenedJointsPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeFastenedJointBaseStrength, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_base_strength.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.fastened_joints.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "fastened-joints", Vec::<String>::new());
    };
    if row.base_strength == payload.new_base_strength {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The fastened_joints block already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { fastened_joints: En1993FastenedJointsDelta::modification(payload.id.clone(), En1993FastenedJointsPatch { base_strength: Some(payload.new_base_strength), ..Default::default() }), ..Default::default() })
}
