//! ↩️ `change-joint-extension-edge` inverse: restores the previous `extension_edge`.

use crate::change_joint_extension_edge::ChangeJointExtensionEdge;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointExtensionEdge, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_extension_edge::ChangeJointExtensionEdge> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointExtensionEdge { id: payload.id.clone(), new_extension_edge: row.extension_edge })],
        None => Vec::new(),
    })
}
