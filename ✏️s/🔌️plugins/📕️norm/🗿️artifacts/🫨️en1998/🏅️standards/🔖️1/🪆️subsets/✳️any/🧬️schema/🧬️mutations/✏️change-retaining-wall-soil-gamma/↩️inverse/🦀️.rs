//! ↩️ `change-retaining-wall-soil-gamma` inverse: restores the previous `soil_gamma`.

use crate::change_retaining_wall_soil_gamma::ChangeRetainingWallSoilGamma;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeRetainingWallSoilGamma, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_retaining_wall_soil_gamma::ChangeRetainingWallSoilGamma> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeRetainingWallSoilGamma, base: &En1998Snapshot) -> Option<ChangeRetainingWallSoilGamma> {
    let r0 = base.retaining_walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeRetainingWallSoilGamma { id: payload.id.clone(), new_soil_gamma: r0.soil_gamma })
}
