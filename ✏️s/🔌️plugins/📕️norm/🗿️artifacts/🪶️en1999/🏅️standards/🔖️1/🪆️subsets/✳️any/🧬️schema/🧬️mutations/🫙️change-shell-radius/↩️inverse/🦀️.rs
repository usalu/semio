//! ↩️ `change-shell-radius` inverse: restores the previous `radius`.

use crate::change_shell_radius::ChangeShellRadius;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeShellRadius, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_shell_radius::ChangeShellRadius> {
    Ok(match base.shells.iter().find(|row| row.id == payload.shell_id) {
        Some(row) => vec![Op::from(ChangeShellRadius { shell_id: payload.shell_id.clone(), new_radius: row.radius })],
        None => Vec::new(),
    })
}
