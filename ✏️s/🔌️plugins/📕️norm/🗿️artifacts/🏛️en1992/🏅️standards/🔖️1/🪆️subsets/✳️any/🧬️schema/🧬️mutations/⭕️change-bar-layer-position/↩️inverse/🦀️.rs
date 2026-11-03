//! ↩️ `change-bar-layer-position` inverse: restores the previous `position`.

use crate::change_bar_layer_position::ChangeBarLayerPosition;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeBarLayerPosition, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bar_layer_position::ChangeBarLayerPosition> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBarLayerPosition, base: &En1992Snapshot) -> Option<ChangeBarLayerPosition> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let r1 = r0.longitudinal.iter().find(|row| row.id == payload.layer_id)?;
    Some(ChangeBarLayerPosition { member_id: payload.member_id.clone(), layer_id: payload.layer_id.clone(), new_position: r1.position })
}
