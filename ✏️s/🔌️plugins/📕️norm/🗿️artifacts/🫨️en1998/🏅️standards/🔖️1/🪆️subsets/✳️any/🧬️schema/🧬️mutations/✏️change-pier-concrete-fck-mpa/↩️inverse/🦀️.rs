//! ↩️ `change-pier-concrete-fck-mpa` inverse: restores the previous `concrete_fck_mpa`.

use crate::change_pier_concrete_fck_mpa::ChangePierConcreteFckMpa;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangePierConcreteFckMpa, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pier_concrete_fck_mpa::ChangePierConcreteFckMpa> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePierConcreteFckMpa, base: &En1998Snapshot) -> Option<ChangePierConcreteFckMpa> {
    let r0 = base.piers.iter().find(|row| row.id == payload.id)?;
    Some(ChangePierConcreteFckMpa { id: payload.id.clone(), new_concrete_fck_mpa: r0.concrete_fck_mpa })
}
