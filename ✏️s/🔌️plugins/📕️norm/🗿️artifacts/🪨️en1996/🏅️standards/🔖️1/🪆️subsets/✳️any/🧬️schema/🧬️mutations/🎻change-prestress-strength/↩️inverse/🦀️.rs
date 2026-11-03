//! ↩️ `change-prestress-strength` inverse: restores the previous `f_pk_pa`.

use crate::change_prestress_strength::ChangePrestressStrength;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangePrestressStrength, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_prestress_strength::ChangePrestressStrength> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePrestressStrength, base: &En1996Snapshot) -> Option<ChangePrestressStrength> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let r1 = r0.prestress.iter().find(|row| row.id == payload.id)?;
    Some(ChangePrestressStrength { wall_id: payload.wall_id.clone(), id: payload.id.clone(), new_f_pk_pa: r1.f_pk_pa })
}
