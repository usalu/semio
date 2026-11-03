//! ✏️ `change-joint-col-moment-rd-nm` diff — patches the one field of the addressed row; a missing target is a `mutation.target-missing`.

use crate::change_joint_col_moment_rd_nm::ChangeJointColMomentRdNm;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998JointDelta, En1998JointPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeJointColMomentRdNm, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(row) = base.joints.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "joint", Vec::<String>::new());
    };
    let patch = En1998JointPatch { column_moment_rd_nm: Some(payload.new_column_moment_rd_nm), ..Default::default() };
    protocol::MutationOutcome::new(En1998Diff { joints: En1998JointDelta::modification(&row.id, patch), ..Default::default() })
}
