//! 🔺️ `change-fastened-joint-fastened joint` diff: patches `fastened joint` of the addressed fastened_joint row.

use crate::change_fastened_joint_kind::ChangeFastenedJointKind;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FastenedJointDelta, En1993FastenedJointPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeFastenedJointKind, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.fastened_joints.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "fastened_joint", Vec::<String>::new());
    };
    if row.kind == payload.new_kind {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The fastened_joint already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { fastened_joints: En1993FastenedJointDelta::modification(payload.id.clone(), En1993FastenedJointPatch { kind: Some(payload.new_kind), ..Default::default() }), ..Default::default() })
}
