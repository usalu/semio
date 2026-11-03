//! ↩️ `change-building-ns-period-s` inverse: restores the previous `nonstructural_period_s`.

use crate::change_building_ns_period_s::ChangeBuildingNsPeriodS;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBuildingNsPeriodS, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_building_ns_period_s::ChangeBuildingNsPeriodS> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBuildingNsPeriodS, base: &En1998Snapshot) -> Option<ChangeBuildingNsPeriodS> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBuildingNsPeriodS { id: payload.id.clone(), new_nonstructural_period_s: r0.nonstructural_period_s })
}
