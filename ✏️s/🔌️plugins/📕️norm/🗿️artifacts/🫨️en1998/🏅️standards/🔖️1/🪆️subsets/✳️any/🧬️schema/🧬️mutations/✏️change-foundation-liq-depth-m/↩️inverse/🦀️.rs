//! ↩️ `change-foundation-liq-depth-m` inverse: restores the previous `liq_depth_m`.

use crate::change_foundation_liq_depth_m::ChangeFoundationLiqDepthM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationLiqDepthM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_liq_depth_m::ChangeFoundationLiqDepthM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFoundationLiqDepthM, base: &En1998Snapshot) -> Option<ChangeFoundationLiqDepthM> {
    let r0 = base.foundations.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFoundationLiqDepthM { id: payload.id.clone(), new_liq_depth_m: r0.liq_depth_m })
}
