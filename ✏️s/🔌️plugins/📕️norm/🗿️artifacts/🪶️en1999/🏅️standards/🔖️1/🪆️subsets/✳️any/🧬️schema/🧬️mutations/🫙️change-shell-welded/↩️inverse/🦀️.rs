//! ↩️ `change-shell-welded` inverse: restores the previous `welded`.

use crate::change_shell_welded::ChangeShellWelded;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeShellWelded, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_shell_welded::ChangeShellWelded> {
    Ok(match base.shells.iter().find(|row| row.id == payload.shell_id) {
        Some(row) => vec![Op::from(ChangeShellWelded { shell_id: payload.shell_id.clone(), new_welded: row.welded })],
        None => Vec::new(),
    })
}
