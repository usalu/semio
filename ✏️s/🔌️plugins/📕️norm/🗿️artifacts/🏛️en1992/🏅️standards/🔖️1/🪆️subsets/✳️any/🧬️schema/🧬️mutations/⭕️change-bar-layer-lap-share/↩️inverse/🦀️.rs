//! ↩️ `change-bar-layer-lap-share` inverse: restores the previous `lap_share`.

use crate::change_bar_layer_lap_share::ChangeBarLayerLapShare;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeBarLayerLapShare, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bar_layer_lap_share::ChangeBarLayerLapShare> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBarLayerLapShare, base: &En1992Snapshot) -> Option<ChangeBarLayerLapShare> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let r1 = r0.longitudinal.iter().find(|row| row.id == payload.layer_id)?;
    Some(ChangeBarLayerLapShare { member_id: payload.member_id.clone(), layer_id: payload.layer_id.clone(), new_lap_share: r1.lap_share })
}
