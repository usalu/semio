//! ↩️ `change-soil-layer-vs-ms` inverse: restores the previous `shear_wave_velocity_m_s`.

use crate::change_soil_layer_vs_m_s::ChangeSoilLayerVsMS;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSoilLayerVsMS, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_soil_layer_vs_m_s::ChangeSoilLayerVsMS> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSoilLayerVsMS, base: &En1998Snapshot) -> Option<ChangeSoilLayerVsMS> {
    let r0 = base.soil_layers.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSoilLayerVsMS { id: payload.id.clone(), new_shear_wave_velocity_m_s: r0.shear_wave_velocity_m_s })
}
