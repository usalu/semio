//! ↩️ `change-shell-action-category` inverse: restores the previous `category`.

use crate::change_shell_action_category::ChangeShellActionCategory;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeShellActionCategory, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_shell_action_category::ChangeShellActionCategory> {
    Ok(match base.shells.iter().find(|row| row.id == payload.shell_id).and_then(|row| row.actions.iter().find(|row| row.id == payload.action_id)) {
        Some(row) => vec![Op::from(ChangeShellActionCategory { shell_id: payload.shell_id.clone(), action_id: payload.action_id.clone(), new_category: row.category })],
        None => Vec::new(),
    })
}
