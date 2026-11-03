//! ↩️ `change-shell-action-source` inverse: restores the previous `source`.

use crate::change_shell_action_source::ChangeShellActionSource;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeShellActionSource, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_shell_action_source::ChangeShellActionSource> {
    Ok(match base.shells.iter().find(|row| row.id == payload.shell_id).and_then(|row| row.actions.iter().find(|row| row.id == payload.action_id)) {
        Some(row) => vec![Op::from(ChangeShellActionSource { shell_id: payload.shell_id.clone(), action_id: payload.action_id.clone(), new_source: row.source })],
        None => Vec::new(),
    })
}
