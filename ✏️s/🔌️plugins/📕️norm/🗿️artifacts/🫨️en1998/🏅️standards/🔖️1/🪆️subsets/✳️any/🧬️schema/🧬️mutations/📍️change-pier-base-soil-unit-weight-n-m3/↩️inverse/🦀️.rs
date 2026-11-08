//! ↩️ `change-pier-base-soil-unit-weight-n-m3` inverse: restores the previous `soil_unit_weight_n_m3`.

use crate::change_pier_base_soil_unit_weight_n_m3::ChangePierBaseSoilUnitWeightNM3;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangePierBaseSoilUnitWeightNM3, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pier_base_soil_unit_weight_n_m3::ChangePierBaseSoilUnitWeightNM3> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePierBaseSoilUnitWeightNM3, base: &En1998Snapshot) -> Option<ChangePierBaseSoilUnitWeightNM3> {
    let r0 = base.pier_bases.iter().find(|row| row.id == payload.id)?;
    Some(ChangePierBaseSoilUnitWeightNM3 { id: payload.id.clone(), new_soil_unit_weight_n_m3: r0.soil_unit_weight_n_m3 })
}
