//! ↩️ `change-soil-layer-n160` inverse: restores the previous `n1_60`.

use crate::change_soil_layer_n1_60::ChangeSoilLayerN160;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSoilLayerN160, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_soil_layer_n1_60::ChangeSoilLayerN160> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSoilLayerN160, base: &En1998Snapshot) -> Option<ChangeSoilLayerN160> {
    let r0 = base.soil_layers.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSoilLayerN160 { id: payload.id.clone(), new_n1_60: r0.n1_60 })
}
