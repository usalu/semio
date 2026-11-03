//! ↩️ `change-prestress-eccentricity` inverse: restores the previous `eccentricity_m`.

use crate::change_prestress_eccentricity::ChangePrestressEccentricity;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangePrestressEccentricity, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_prestress_eccentricity::ChangePrestressEccentricity> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePrestressEccentricity, base: &En1996Snapshot) -> Option<ChangePrestressEccentricity> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let r1 = r0.prestress.iter().find(|row| row.id == payload.id)?;
    Some(ChangePrestressEccentricity { wall_id: payload.wall_id.clone(), id: payload.id.clone(), new_eccentricity_m: r1.eccentricity_m })
}
