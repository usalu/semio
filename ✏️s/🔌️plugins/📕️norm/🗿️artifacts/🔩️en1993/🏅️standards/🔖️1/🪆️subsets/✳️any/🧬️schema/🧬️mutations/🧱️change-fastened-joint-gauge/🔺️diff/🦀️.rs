//! 🔺️ `change-fastened-joint-gauge` diff: patches `gauge` of the addressed fastened_joints block row.

use crate::change_fastened_joint_gauge::ChangeFastenedJointGauge;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FastenedJointsDelta, En1993FastenedJointsPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeFastenedJointGauge, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_gauge.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.fastened_joints.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "fastened-joints", Vec::<String>::new());
    };
    if row.gauge == payload.new_gauge {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The fastened_joints block already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { fastened_joints: En1993FastenedJointsDelta::modification(payload.id.clone(), En1993FastenedJointsPatch { gauge: Some(payload.new_gauge), ..Default::default() }), ..Default::default() })
}
