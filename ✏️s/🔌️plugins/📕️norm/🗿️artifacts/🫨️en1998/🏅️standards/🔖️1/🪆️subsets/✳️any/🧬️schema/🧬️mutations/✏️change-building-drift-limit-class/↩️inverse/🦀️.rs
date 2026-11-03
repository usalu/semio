//! ↩️ `change-building-drift-limit-class` inverse: restores the previous `drift_limit_class`.

use crate::change_building_drift_limit_class::ChangeBuildingDriftLimitClass;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBuildingDriftLimitClass, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_building_drift_limit_class::ChangeBuildingDriftLimitClass> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBuildingDriftLimitClass, base: &En1998Snapshot) -> Option<ChangeBuildingDriftLimitClass> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBuildingDriftLimitClass { id: payload.id.clone(), new_drift_limit_class: r0.drift_limit_class })
}
