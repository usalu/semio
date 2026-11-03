//! ↩️ `change-action-vy` inverse: restores the previous `action.vy`.

use crate::change_action_vy::ChangeActionVy;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeActionVy, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_action_vy::ChangeActionVy> {
    Ok(match base.member_actions.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeActionVy { id: payload.id.clone(), new_action_vy: row.action.vy })],
        None => Vec::new(),
    })
}
