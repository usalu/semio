//! ↩️ `change-pier-modulus-mpa` inverse: restores the previous `modulus_mpa`.

use crate::change_pier_modulus_mpa::ChangePierModulusMpa;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangePierModulusMpa, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pier_modulus_mpa::ChangePierModulusMpa> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePierModulusMpa, base: &En1998Snapshot) -> Option<ChangePierModulusMpa> {
    let r0 = base.piers.iter().find(|row| row.id == payload.id)?;
    Some(ChangePierModulusMpa { id: payload.id.clone(), new_modulus_mpa: r0.modulus_mpa })
}
