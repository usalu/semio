//! ↩️ `change-bridge-correlated-occupancy` inverse: restores the previous `correlated_occupancy`.

use crate::change_bridge_correlated_occupancy::ChangeBridgeCorrelatedOccupancy;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeCorrelatedOccupancy, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_correlated_occupancy::ChangeBridgeCorrelatedOccupancy> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBridgeCorrelatedOccupancy, base: &En1998Snapshot) -> Option<ChangeBridgeCorrelatedOccupancy> {
    let r0 = base.bridges.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBridgeCorrelatedOccupancy { id: payload.id.clone(), new_correlated_occupancy: r0.correlated_occupancy })
}
