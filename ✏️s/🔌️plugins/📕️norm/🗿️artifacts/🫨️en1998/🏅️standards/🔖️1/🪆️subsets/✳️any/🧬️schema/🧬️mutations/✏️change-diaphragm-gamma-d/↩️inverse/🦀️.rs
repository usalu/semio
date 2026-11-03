//! ↩️ `change-diaphragm-gamma-d` inverse: restores the previous `gamma_d`.

use crate::change_diaphragm_gamma_d::ChangeDiaphragmGammaD;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeDiaphragmGammaD, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_diaphragm_gamma_d::ChangeDiaphragmGammaD> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeDiaphragmGammaD, base: &En1998Snapshot) -> Option<ChangeDiaphragmGammaD> {
    let r0 = base.diaphragms.iter().find(|row| row.id == payload.id)?;
    Some(ChangeDiaphragmGammaD { id: payload.id.clone(), new_gamma_d: r0.gamma_d })
}
