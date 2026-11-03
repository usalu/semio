//! ↩️ `change-shell-action-myk` inverse: restores the previous `m_y_k`.

use crate::change_shell_action_m_y_k::ChangeShellActionMYK;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeShellActionMYK, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_shell_action_m_y_k::ChangeShellActionMYK> {
    Ok(match base.shells.iter().find(|row| row.id == payload.shell_id).and_then(|row| row.actions.iter().find(|row| row.id == payload.action_id)) {
        Some(row) => vec![Op::from(ChangeShellActionMYK { shell_id: payload.shell_id.clone(), action_id: payload.action_id.clone(), new_m_y_k: row.m_y_k })],
        None => Vec::new(),
    })
}
