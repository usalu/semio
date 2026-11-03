//! ↩️ `change-shell-action-gk-line` inverse: restores the previous `g_k_line`.

use crate::change_shell_action_g_k_line::ChangeShellActionGKLine;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeShellActionGKLine, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_shell_action_g_k_line::ChangeShellActionGKLine> {
    Ok(match base.shells.iter().find(|row| row.id == payload.shell_id).and_then(|row| row.actions.iter().find(|row| row.id == payload.action_id)) {
        Some(row) => vec![Op::from(ChangeShellActionGKLine { shell_id: payload.shell_id.clone(), action_id: payload.action_id.clone(), new_g_k_line: row.g_k_line })],
        None => Vec::new(),
    })
}
