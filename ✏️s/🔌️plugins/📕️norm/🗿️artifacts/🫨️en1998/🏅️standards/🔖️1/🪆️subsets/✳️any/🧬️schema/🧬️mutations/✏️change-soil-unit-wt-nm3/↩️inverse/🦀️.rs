//! ↩️ `change-soil-unit-wt-nm3` inverse: restores the previous `unit_weight_n_m3`.

use crate::change_soil_unit_wt_n_m3::ChangeSoilUnitWtNM3;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSoilUnitWtNM3, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_soil_unit_wt_n_m3::ChangeSoilUnitWtNM3> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSoilUnitWtNM3, base: &En1998Snapshot) -> Option<ChangeSoilUnitWtNM3> {
    let r0 = base.soil_layers.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSoilUnitWtNM3 { id: payload.id.clone(), new_unit_weight_n_m3: r0.unit_weight_n_m3 })
}
