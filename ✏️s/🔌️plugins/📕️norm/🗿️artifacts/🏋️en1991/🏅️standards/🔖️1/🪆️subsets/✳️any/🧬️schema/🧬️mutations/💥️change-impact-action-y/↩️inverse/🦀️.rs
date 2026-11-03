//! ↩️ `change-impact-action-y` inverse: restores the previous `y`.

use crate::change_impact_action_y::ChangeImpactActionY;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeImpactActionY, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_impact_action_y::ChangeImpactActionY> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeImpactActionY, base: &En1991Snapshot) -> Option<ChangeImpactActionY> {
    let r0 = base.impact_actions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeImpactActionY { id: payload.id.clone(), new_y: r0.y })
}
