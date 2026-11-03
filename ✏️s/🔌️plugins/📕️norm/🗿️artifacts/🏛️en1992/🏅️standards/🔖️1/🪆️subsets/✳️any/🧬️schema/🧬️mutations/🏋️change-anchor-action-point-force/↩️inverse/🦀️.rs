//! ↩️ `change-anchor-action-point-force` inverse: restores the previous `point_force`.

use crate::change_anchor_action_point_force::ChangeAnchorActionPointForce;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeAnchorActionPointForce, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_anchor_action_point_force::ChangeAnchorActionPointForce> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAnchorActionPointForce, base: &En1992Snapshot) -> Option<ChangeAnchorActionPointForce> {
    let r0 = base.anchors.iter().find(|row| row.id == payload.anchor_id)?;
    let r1 = r0.actions.iter().find(|row| row.id == payload.action_id)?;
    Some(ChangeAnchorActionPointForce { anchor_id: payload.anchor_id.clone(), action_id: payload.action_id.clone(), new_point_force: r1.point_force })
}
