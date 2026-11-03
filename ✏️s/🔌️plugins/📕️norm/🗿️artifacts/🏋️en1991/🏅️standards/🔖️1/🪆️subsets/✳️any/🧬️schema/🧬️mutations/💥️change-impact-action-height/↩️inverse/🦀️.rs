//! ↩️ `change-impact-action-height` inverse: restores the previous `height`.

use crate::change_impact_action_height::ChangeImpactActionHeight;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeImpactActionHeight, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_impact_action_height::ChangeImpactActionHeight> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeImpactActionHeight, base: &En1991Snapshot) -> Option<ChangeImpactActionHeight> {
    let r0 = base.impact_actions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeImpactActionHeight { id: payload.id.clone(), new_height: r0.height })
}
