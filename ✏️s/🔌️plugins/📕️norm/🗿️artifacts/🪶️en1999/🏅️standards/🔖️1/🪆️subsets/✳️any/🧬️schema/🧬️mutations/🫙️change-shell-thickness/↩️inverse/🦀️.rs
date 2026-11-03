//! ↩️ `change-shell-thickness` inverse: restores the previous `thickness`.

use crate::change_shell_thickness::ChangeShellThickness;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeShellThickness, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_shell_thickness::ChangeShellThickness> {
    Ok(match base.shells.iter().find(|row| row.id == payload.shell_id) {
        Some(row) => vec![Op::from(ChangeShellThickness { shell_id: payload.shell_id.clone(), new_thickness: row.thickness })],
        None => Vec::new(),
    })
}
