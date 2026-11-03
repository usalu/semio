//! ↩️ `change-shell-material-id` inverse: restores the previous `material_id`.

use crate::change_shell_material_id::ChangeShellMaterialId;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeShellMaterialId, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_shell_material_id::ChangeShellMaterialId> {
    Ok(match base.shells.iter().find(|row| row.id == payload.shell_id) {
        Some(row) => vec![Op::from(ChangeShellMaterialId { shell_id: payload.shell_id.clone(), new_material_id: row.material_id.clone() })],
        None => Vec::new(),
    })
}
