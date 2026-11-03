//! ↩️ `change-foundation-liq-unit-wt-nm3` inverse: restores the previous `liq_unit_weight_n_m3`.

use crate::change_foundation_liq_unit_wt_n_m3::ChangeFoundationLiqUnitWtNM3;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationLiqUnitWtNM3, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_liq_unit_wt_n_m3::ChangeFoundationLiqUnitWtNM3> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFoundationLiqUnitWtNM3, base: &En1998Snapshot) -> Option<ChangeFoundationLiqUnitWtNM3> {
    let r0 = base.foundations.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFoundationLiqUnitWtNM3 { id: payload.id.clone(), new_liq_unit_weight_n_m3: r0.liq_unit_weight_n_m3 })
}
