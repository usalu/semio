//! ↩️ `change-foundation-liq-layer-m` inverse: restores the previous `liq_layer_thickness_m`.

use crate::change_foundation_liq_layer_m::ChangeFoundationLiqLayerM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationLiqLayerM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_liq_layer_m::ChangeFoundationLiqLayerM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFoundationLiqLayerM, base: &En1998Snapshot) -> Option<ChangeFoundationLiqLayerM> {
    let r0 = base.foundations.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFoundationLiqLayerM { id: payload.id.clone(), new_liq_layer_thickness_m: r0.liq_layer_thickness_m })
}
