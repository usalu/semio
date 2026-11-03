//! ↩️ `change-shell-action-vyk` inverse: restores the previous `v_y_k`.

use crate::change_shell_action_v_y_k::ChangeShellActionVYK;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeShellActionVYK, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_shell_action_v_y_k::ChangeShellActionVYK> {
    Ok(match base.shells.iter().find(|row| row.id == payload.shell_id).and_then(|row| row.actions.iter().find(|row| row.id == payload.action_id)) {
        Some(row) => vec![Op::from(ChangeShellActionVYK { shell_id: payload.shell_id.clone(), action_id: payload.action_id.clone(), new_v_y_k: row.v_y_k })],
        None => Vec::new(),
    })
}
