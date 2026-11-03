//! ↩️ `change-bar-layer-lap-length` inverse: restores the previous `lap_length`.

use crate::change_bar_layer_lap_length::ChangeBarLayerLapLength;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeBarLayerLapLength, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bar_layer_lap_length::ChangeBarLayerLapLength> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBarLayerLapLength, base: &En1992Snapshot) -> Option<ChangeBarLayerLapLength> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let r1 = r0.longitudinal.iter().find(|row| row.id == payload.layer_id)?;
    Some(ChangeBarLayerLapLength { member_id: payload.member_id.clone(), layer_id: payload.layer_id.clone(), new_lap_length: r1.lap_length })
}
