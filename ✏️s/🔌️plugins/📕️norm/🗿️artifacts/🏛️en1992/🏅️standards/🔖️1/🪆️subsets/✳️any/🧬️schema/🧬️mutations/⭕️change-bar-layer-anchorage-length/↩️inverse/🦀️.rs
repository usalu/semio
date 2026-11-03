//! ↩️ `change-bar-layer-anchorage-length` inverse: restores the previous `anchorage_length`.

use crate::change_bar_layer_anchorage_length::ChangeBarLayerAnchorageLength;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeBarLayerAnchorageLength, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bar_layer_anchorage_length::ChangeBarLayerAnchorageLength> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBarLayerAnchorageLength, base: &En1992Snapshot) -> Option<ChangeBarLayerAnchorageLength> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let r1 = r0.longitudinal.iter().find(|row| row.id == payload.layer_id)?;
    Some(ChangeBarLayerAnchorageLength { member_id: payload.member_id.clone(), layer_id: payload.layer_id.clone(), new_anchorage_length: r1.anchorage_length })
}
