//! ↩️ `change-impact-action-x` inverse: restores the previous `x`.

use crate::change_impact_action_x::ChangeImpactActionX;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeImpactActionX, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_impact_action_x::ChangeImpactActionX> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeImpactActionX, base: &En1991Snapshot) -> Option<ChangeImpactActionX> {
    let r0 = base.impact_actions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeImpactActionX { id: payload.id.clone(), new_x: r0.x })
}
