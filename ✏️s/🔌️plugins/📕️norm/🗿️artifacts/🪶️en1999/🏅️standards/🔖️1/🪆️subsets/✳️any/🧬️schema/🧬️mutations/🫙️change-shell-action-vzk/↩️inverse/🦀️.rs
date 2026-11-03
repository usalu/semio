//! ↩️ `change-shell-action-vzk` inverse: restores the previous `v_z_k`.

use crate::change_shell_action_v_z_k::ChangeShellActionVZK;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeShellActionVZK, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_shell_action_v_z_k::ChangeShellActionVZK> {
    Ok(match base.shells.iter().find(|row| row.id == payload.shell_id).and_then(|row| row.actions.iter().find(|row| row.id == payload.action_id)) {
        Some(row) => vec![Op::from(ChangeShellActionVZK { shell_id: payload.shell_id.clone(), action_id: payload.action_id.clone(), new_v_z_k: row.v_z_k })],
        None => Vec::new(),
    })
}
