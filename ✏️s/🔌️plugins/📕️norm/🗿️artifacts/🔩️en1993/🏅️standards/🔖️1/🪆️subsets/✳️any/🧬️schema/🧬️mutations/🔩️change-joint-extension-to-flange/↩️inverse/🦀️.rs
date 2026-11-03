//! ↩️ `change-joint-extension-to-flange` inverse: restores the previous `extension_to_flange`.

use crate::change_joint_extension_to_flange::ChangeJointExtensionToFlange;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointExtensionToFlange, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_extension_to_flange::ChangeJointExtensionToFlange> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointExtensionToFlange { id: payload.id.clone(), new_extension_to_flange: row.extension_to_flange })],
        None => Vec::new(),
    })
}
