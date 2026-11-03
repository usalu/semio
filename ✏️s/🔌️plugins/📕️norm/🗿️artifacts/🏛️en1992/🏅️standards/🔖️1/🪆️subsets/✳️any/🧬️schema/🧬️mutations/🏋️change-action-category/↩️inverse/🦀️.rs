//! ↩️ `change-action-category` inverse: restores the previous `category`.

use crate::change_action_category::ChangeActionCategory;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeActionCategory, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_action_category::ChangeActionCategory> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeActionCategory, base: &En1992Snapshot) -> Option<ChangeActionCategory> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let r1 = r0.actions.iter().find(|row| row.id == payload.action_id)?;
    Some(ChangeActionCategory { member_id: payload.member_id.clone(), action_id: payload.action_id.clone(), new_category: r1.category })
}
