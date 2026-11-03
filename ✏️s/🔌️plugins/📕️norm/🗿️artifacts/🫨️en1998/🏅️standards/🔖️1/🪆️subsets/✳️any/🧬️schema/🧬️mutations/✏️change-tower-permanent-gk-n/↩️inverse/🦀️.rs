//! ↩️ `change-tower-permanent-gk-n` inverse: restores the previous `permanent_gk_n`.

use crate::change_tower_permanent_gk_n::ChangeTowerPermanentGkN;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeTowerPermanentGkN, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tower_permanent_gk_n::ChangeTowerPermanentGkN> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTowerPermanentGkN, base: &En1998Snapshot) -> Option<ChangeTowerPermanentGkN> {
    let r0 = base.towers.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTowerPermanentGkN { id: payload.id.clone(), new_permanent_gk_n: r0.permanent_gk_n })
}
