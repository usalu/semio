//! ↩️ `change-pier-stiffness-basis` inverse: restores the previous `stiffness_basis`.

use crate::change_pier_stiffness_basis::ChangePierStiffnessBasis;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangePierStiffnessBasis, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pier_stiffness_basis::ChangePierStiffnessBasis> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePierStiffnessBasis, base: &En1998Snapshot) -> Option<ChangePierStiffnessBasis> {
    let r0 = base.piers.iter().find(|row| row.id == payload.id)?;
    Some(ChangePierStiffnessBasis { id: payload.id.clone(), new_stiffness_basis: r0.stiffness_basis })
}
