//! ↩️ `change-action-mz` inverse: restores the previous `action.mz`.

use crate::change_action_mz::ChangeActionMz;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeActionMz, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_action_mz::ChangeActionMz> {
    Ok(match base.member_actions.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeActionMz { id: payload.id.clone(), new_action_mz: row.action.mz })],
        None => Vec::new(),
    })
}
