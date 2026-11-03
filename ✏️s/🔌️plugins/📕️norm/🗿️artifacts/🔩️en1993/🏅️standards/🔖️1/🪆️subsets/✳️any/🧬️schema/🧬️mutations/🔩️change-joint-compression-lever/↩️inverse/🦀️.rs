//! ↩️ `change-joint-compression-lever` inverse: restores the previous `compression_lever`.

use crate::change_joint_compression_lever::ChangeJointCompressionLever;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointCompressionLever, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_compression_lever::ChangeJointCompressionLever> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointCompressionLever { id: payload.id.clone(), new_compression_lever: row.compression_lever })],
        None => Vec::new(),
    })
}
