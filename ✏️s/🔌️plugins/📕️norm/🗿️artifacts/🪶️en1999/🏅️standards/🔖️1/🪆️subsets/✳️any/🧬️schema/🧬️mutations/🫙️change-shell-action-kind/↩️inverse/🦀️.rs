//! ↩️ `change-shell-action-kind` inverse: restores the previous `kind`.

use crate::change_shell_action_kind::ChangeShellActionKind;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeShellActionKind, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_shell_action_kind::ChangeShellActionKind> {
    Ok(match base.shells.iter().find(|row| row.id == payload.shell_id).and_then(|row| row.actions.iter().find(|row| row.id == payload.action_id)) {
        Some(row) => vec![Op::from(ChangeShellActionKind { shell_id: payload.shell_id.clone(), action_id: payload.action_id.clone(), new_kind: row.kind })],
        None => Vec::new(),
    })
}
