//! ↩️ `change-row-extension-edge` inverse: restores the previous `extension_edge`.

use crate::change_row_extension_edge::ChangeRowExtensionEdge;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeRowExtensionEdge, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_row_extension_edge::ChangeRowExtensionEdge> {
    Ok(match base.joint_rows.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeRowExtensionEdge { id: payload.id.clone(), new_extension_edge: row.extension_edge })],
        None => Vec::new(),
    })
}
