//! ↩️ `change-storey-drift-ym` inverse: restores the previous `drift_y_m`.

use crate::change_storey_drift_y_m::ChangeStoreyDriftYM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeStoreyDriftYM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_storey_drift_y_m::ChangeStoreyDriftYM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeStoreyDriftYM, base: &En1998Snapshot) -> Option<ChangeStoreyDriftYM> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let r1 = r0.storeys.iter().find(|row| row.id == payload.id)?;
    Some(ChangeStoreyDriftYM { building_id: payload.building_id.clone(), id: payload.id.clone(), new_drift_y_m: r1.drift_y_m })
}
