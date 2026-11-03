//! ↩️ `change-soil-layer-foundation-id` inverse: restores the previous `foundation_id`.

use crate::change_soil_layer_foundation_id::ChangeSoilLayerFoundationId;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSoilLayerFoundationId, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_soil_layer_foundation_id::ChangeSoilLayerFoundationId> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSoilLayerFoundationId, base: &En1998Snapshot) -> Option<ChangeSoilLayerFoundationId> {
    let r0 = base.soil_layers.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSoilLayerFoundationId { id: payload.id.clone(), new_foundation_id: r0.foundation_id.clone() })
}
