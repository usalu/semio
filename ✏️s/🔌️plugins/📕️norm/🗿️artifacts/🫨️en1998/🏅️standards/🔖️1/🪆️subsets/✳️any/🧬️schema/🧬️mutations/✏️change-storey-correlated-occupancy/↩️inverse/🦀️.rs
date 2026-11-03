//! ↩️ `change-storey-correlated-occupancy` inverse: restores the previous `correlated_occupancy`.

use crate::change_storey_correlated_occupancy::ChangeStoreyCorrelatedOccupancy;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeStoreyCorrelatedOccupancy, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_storey_correlated_occupancy::ChangeStoreyCorrelatedOccupancy> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeStoreyCorrelatedOccupancy, base: &En1998Snapshot) -> Option<ChangeStoreyCorrelatedOccupancy> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let r1 = r0.storeys.iter().find(|row| row.id == payload.id)?;
    Some(ChangeStoreyCorrelatedOccupancy { building_id: payload.building_id.clone(), id: payload.id.clone(), new_correlated_occupancy: r1.correlated_occupancy })
}
