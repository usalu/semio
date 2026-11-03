//! ↩️ `change-action-n` inverse: restores the previous `action.n`.

use crate::change_action_n::ChangeActionN;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeActionN, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_action_n::ChangeActionN> {
    Ok(match base.member_actions.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeActionN { id: payload.id.clone(), new_action_n: row.action.n })],
        None => Vec::new(),
    })
}
