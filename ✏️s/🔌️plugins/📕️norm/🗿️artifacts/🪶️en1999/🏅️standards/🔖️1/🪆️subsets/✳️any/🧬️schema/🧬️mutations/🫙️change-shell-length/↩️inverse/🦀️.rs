//! ↩️ `change-shell-length` inverse: restores the previous `length`.

use crate::change_shell_length::ChangeShellLength;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeShellLength, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_shell_length::ChangeShellLength> {
    Ok(match base.shells.iter().find(|row| row.id == payload.shell_id) {
        Some(row) => vec![Op::from(ChangeShellLength { shell_id: payload.shell_id.clone(), new_length: row.length })],
        None => Vec::new(),
    })
}
