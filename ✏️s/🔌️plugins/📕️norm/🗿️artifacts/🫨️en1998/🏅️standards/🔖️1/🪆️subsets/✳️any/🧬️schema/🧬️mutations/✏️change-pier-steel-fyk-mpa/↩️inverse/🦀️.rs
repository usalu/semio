//! ↩️ `change-pier-steel-fyk-mpa` inverse: restores the previous `steel_fyk_mpa`.

use crate::change_pier_steel_fyk_mpa::ChangePierSteelFykMpa;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangePierSteelFykMpa, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pier_steel_fyk_mpa::ChangePierSteelFykMpa> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePierSteelFykMpa, base: &En1998Snapshot) -> Option<ChangePierSteelFykMpa> {
    let r0 = base.piers.iter().find(|row| row.id == payload.id)?;
    Some(ChangePierSteelFykMpa { id: payload.id.clone(), new_steel_fyk_mpa: r0.steel_fyk_mpa })
}
