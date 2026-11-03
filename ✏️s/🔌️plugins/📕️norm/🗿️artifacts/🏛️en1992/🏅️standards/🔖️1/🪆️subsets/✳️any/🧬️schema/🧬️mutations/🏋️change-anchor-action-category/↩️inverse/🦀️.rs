//! ↩️ `change-anchor-action-category` inverse: restores the previous `category`.

use crate::change_anchor_action_category::ChangeAnchorActionCategory;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeAnchorActionCategory, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_anchor_action_category::ChangeAnchorActionCategory> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAnchorActionCategory, base: &En1992Snapshot) -> Option<ChangeAnchorActionCategory> {
    let r0 = base.anchors.iter().find(|row| row.id == payload.anchor_id)?;
    let r1 = r0.actions.iter().find(|row| row.id == payload.action_id)?;
    Some(ChangeAnchorActionCategory { anchor_id: payload.anchor_id.clone(), action_id: payload.action_id.clone(), new_category: r1.category })
}
