//! ↩️ `change-silo-permanent-gk-n` inverse: restores the previous `permanent_gk_n`.

use crate::change_silo_permanent_gk_n::ChangeSiloPermanentGkN;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSiloPermanentGkN, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_permanent_gk_n::ChangeSiloPermanentGkN> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSiloPermanentGkN, base: &En1998Snapshot) -> Option<ChangeSiloPermanentGkN> {
    let r0 = base.silos.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSiloPermanentGkN { id: payload.id.clone(), new_permanent_gk_n: r0.permanent_gk_n })
}
