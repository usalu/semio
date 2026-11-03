//! ↩️ `change-eccentricity-bottom` inverse: restores the previous `eccentricity_bottom_m`.

use crate::change_eccentricity_bottom::ChangeEccentricityBottom;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeEccentricityBottom, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_eccentricity_bottom::ChangeEccentricityBottom> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeEccentricityBottom, base: &En1996Snapshot) -> Option<ChangeEccentricityBottom> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeEccentricityBottom { id: payload.id.clone(), new_eccentricity_bottom_m: r0.eccentricity_bottom_m })
}
