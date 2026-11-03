//! ↩️ `change-row-extension-to-flange` inverse: restores the previous `extension_to_flange`.

use crate::change_row_extension_to_flange::ChangeRowExtensionToFlange;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeRowExtensionToFlange, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_row_extension_to_flange::ChangeRowExtensionToFlange> {
    Ok(match base.joint_rows.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeRowExtensionToFlange { id: payload.id.clone(), new_extension_to_flange: row.extension_to_flange })],
        None => Vec::new(),
    })
}
