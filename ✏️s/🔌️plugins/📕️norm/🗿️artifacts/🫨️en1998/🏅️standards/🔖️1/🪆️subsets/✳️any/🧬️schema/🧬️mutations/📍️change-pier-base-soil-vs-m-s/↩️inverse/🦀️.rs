//! ↩️ `change-pier-base-soil-vs-m-s` inverse: restores the previous `soil_vs_m_s`.

use crate::change_pier_base_soil_vs_m_s::ChangePierBaseSoilVsMS;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangePierBaseSoilVsMS, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pier_base_soil_vs_m_s::ChangePierBaseSoilVsMS> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePierBaseSoilVsMS, base: &En1998Snapshot) -> Option<ChangePierBaseSoilVsMS> {
    let r0 = base.pier_bases.iter().find(|row| row.id == payload.id)?;
    Some(ChangePierBaseSoilVsMS { id: payload.id.clone(), new_soil_vs_m_s: r0.soil_vs_m_s })
}
