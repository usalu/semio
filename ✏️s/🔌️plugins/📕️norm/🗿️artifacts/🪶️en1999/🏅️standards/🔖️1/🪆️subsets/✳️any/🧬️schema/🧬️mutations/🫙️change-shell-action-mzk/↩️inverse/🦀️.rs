//! ↩️ `change-shell-action-mzk` inverse: restores the previous `m_z_k`.

use crate::change_shell_action_m_z_k::ChangeShellActionMZK;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeShellActionMZK, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_shell_action_m_z_k::ChangeShellActionMZK> {
    Ok(match base.shells.iter().find(|row| row.id == payload.shell_id).and_then(|row| row.actions.iter().find(|row| row.id == payload.action_id)) {
        Some(row) => vec![Op::from(ChangeShellActionMZK { shell_id: payload.shell_id.clone(), action_id: payload.action_id.clone(), new_m_z_k: row.m_z_k })],
        None => Vec::new(),
    })
}
