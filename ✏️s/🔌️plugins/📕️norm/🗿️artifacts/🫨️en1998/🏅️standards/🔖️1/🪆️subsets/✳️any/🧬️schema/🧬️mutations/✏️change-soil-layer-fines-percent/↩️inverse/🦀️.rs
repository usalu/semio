//! ↩️ `change-soil-layer-fines-percent` inverse: restores the previous `fines_percent`.

use crate::change_soil_layer_fines_percent::ChangeSoilLayerFinesPercent;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSoilLayerFinesPercent, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_soil_layer_fines_percent::ChangeSoilLayerFinesPercent> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSoilLayerFinesPercent, base: &En1998Snapshot) -> Option<ChangeSoilLayerFinesPercent> {
    let r0 = base.soil_layers.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSoilLayerFinesPercent { id: payload.id.clone(), new_fines_percent: r0.fines_percent })
}
