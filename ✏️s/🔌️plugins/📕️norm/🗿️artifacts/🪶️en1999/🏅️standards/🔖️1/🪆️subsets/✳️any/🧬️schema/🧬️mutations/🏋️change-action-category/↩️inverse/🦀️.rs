//! ↩️ `change-action-category` inverse: restores the previous `category`.

use crate::change_action_category::ChangeActionCategory;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeActionCategory, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_action_category::ChangeActionCategory> {
    Ok(match base.members.iter().find(|row| row.id == payload.member_id).and_then(|row| row.actions.iter().find(|row| row.id == payload.action_id)) {
        Some(row) => vec![Op::from(ChangeActionCategory { member_id: payload.member_id.clone(), action_id: payload.action_id.clone(), new_category: row.category })],
        None => Vec::new(),
    })
}
