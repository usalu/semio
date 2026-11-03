//! ↩️ `change-eccentricity-top` inverse: restores the previous `eccentricity_top_m`.

use crate::change_eccentricity_top::ChangeEccentricityTop;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeEccentricityTop, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_eccentricity_top::ChangeEccentricityTop> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeEccentricityTop, base: &En1996Snapshot) -> Option<ChangeEccentricityTop> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeEccentricityTop { id: payload.id.clone(), new_eccentricity_top_m: r0.eccentricity_top_m })
}
