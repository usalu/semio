//! ↩️ `change-tank-permanent-gk-n` inverse: restores the previous `permanent_gk_n`.

use crate::change_tank_permanent_gk_n::ChangeTankPermanentGkN;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeTankPermanentGkN, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tank_permanent_gk_n::ChangeTankPermanentGkN> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTankPermanentGkN, base: &En1998Snapshot) -> Option<ChangeTankPermanentGkN> {
    let r0 = base.tanks.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTankPermanentGkN { id: payload.id.clone(), new_permanent_gk_n: r0.permanent_gk_n })
}
