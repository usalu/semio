//! ↩️ `remove-shell-action` inverse.

use crate::remove_shell_action::RemoveShellAction;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::insert_shell_action;

pub fn inverse<Op>(payload: &RemoveShellAction, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_shell_action::InsertShellAction> {
    Ok((|| {
    match base.shells.iter().find(|row| row.id == payload.shell_id).and_then(|parent| parent.actions.iter().enumerate().find(|(_, row)| row.id == payload.id)) {
        Some((index, row)) => vec![Op::from(insert_shell_action::InsertShellAction { shell_id: payload.shell_id.clone(), index: index as u32, action: row.clone() })],
        None => Vec::new(),
    }
    })())
}
