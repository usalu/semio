//! ↩️ `change-f-yd` inverse: restores the previous `f_yd_pa`.

use crate::change_f_yd::ChangeFYd;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeFYd, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_f_yd::ChangeFYd> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFYd, base: &En1996Snapshot) -> Option<ChangeFYd> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFYd { id: payload.id.clone(), new_f_yd_pa: r0.f_yd_pa })
}
