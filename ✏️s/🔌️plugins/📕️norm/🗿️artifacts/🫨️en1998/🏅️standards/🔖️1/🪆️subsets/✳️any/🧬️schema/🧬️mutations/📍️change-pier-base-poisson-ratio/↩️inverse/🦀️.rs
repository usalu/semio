//! ↩️ `change-pier-base-poisson-ratio` inverse: restores the previous `poisson_ratio`.

use crate::change_pier_base_poisson_ratio::ChangePierBasePoissonRatio;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangePierBasePoissonRatio, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pier_base_poisson_ratio::ChangePierBasePoissonRatio> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePierBasePoissonRatio, base: &En1998Snapshot) -> Option<ChangePierBasePoissonRatio> {
    let r0 = base.pier_bases.iter().find(|row| row.id == payload.id)?;
    Some(ChangePierBasePoissonRatio { id: payload.id.clone(), new_poisson_ratio: r0.poisson_ratio })
}
