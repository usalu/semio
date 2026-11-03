//! ↩️ `change-bar-point-layer-id` inverse: restores the previous `layer_id`.

use crate::change_bar_point_layer_id::ChangeBarPointLayerId;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeBarPointLayerId, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bar_point_layer_id::ChangeBarPointLayerId> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBarPointLayerId, base: &En1992Snapshot) -> Option<ChangeBarPointLayerId> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let r1 = r0.bars.iter().find(|row| row.id == payload.bar_id)?;
    Some(ChangeBarPointLayerId { member_id: payload.member_id.clone(), bar_id: payload.bar_id.clone(), new_layer_id: r1.layer_id.clone() })
}
