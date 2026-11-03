//! ↩️ `change-soil-layer-thickness-m` inverse: restores the previous `thickness_m`.

use crate::change_soil_layer_thickness_m::ChangeSoilLayerThicknessM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSoilLayerThicknessM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_soil_layer_thickness_m::ChangeSoilLayerThicknessM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSoilLayerThicknessM, base: &En1998Snapshot) -> Option<ChangeSoilLayerThicknessM> {
    let r0 = base.soil_layers.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSoilLayerThicknessM { id: payload.id.clone(), new_thickness_m: r0.thickness_m })
}
