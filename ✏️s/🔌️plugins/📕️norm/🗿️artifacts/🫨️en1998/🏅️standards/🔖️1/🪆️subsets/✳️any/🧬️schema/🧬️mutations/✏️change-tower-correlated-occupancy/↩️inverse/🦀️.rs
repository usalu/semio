//! ↩️ `change-tower-correlated-occupancy` inverse: restores the previous `correlated_occupancy`.

use crate::change_tower_correlated_occupancy::ChangeTowerCorrelatedOccupancy;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeTowerCorrelatedOccupancy, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tower_correlated_occupancy::ChangeTowerCorrelatedOccupancy> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTowerCorrelatedOccupancy, base: &En1998Snapshot) -> Option<ChangeTowerCorrelatedOccupancy> {
    let r0 = base.towers.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTowerCorrelatedOccupancy { id: payload.id.clone(), new_correlated_occupancy: r0.correlated_occupancy })
}
