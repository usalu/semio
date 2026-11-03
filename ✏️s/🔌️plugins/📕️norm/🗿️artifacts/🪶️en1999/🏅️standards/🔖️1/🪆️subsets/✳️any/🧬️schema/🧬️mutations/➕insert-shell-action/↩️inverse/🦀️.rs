//! ↩️ `insert-shell-action` inverse.

use crate::insert_shell_action::InsertShellAction;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::remove_shell_action;

pub fn inverse<Op>(payload: &InsertShellAction, _base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_shell_action::RemoveShellAction> {
    Ok((|| {
    vec![Op::from(remove_shell_action::RemoveShellAction { shell_id: payload.shell_id.clone(), id: payload.action.id.clone() })]
    })())
}
