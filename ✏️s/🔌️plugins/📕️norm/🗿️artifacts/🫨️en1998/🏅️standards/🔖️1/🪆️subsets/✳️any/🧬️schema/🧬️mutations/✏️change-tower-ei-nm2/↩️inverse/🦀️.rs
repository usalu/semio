//! ↩️ `change-tower-ei-nm2` inverse: restores the previous `ei_nm2`.

use crate::change_tower_ei_nm2::ChangeTowerEiNm2;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeTowerEiNm2, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tower_ei_nm2::ChangeTowerEiNm2> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTowerEiNm2, base: &En1998Snapshot) -> Option<ChangeTowerEiNm2> {
    let r0 = base.towers.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTowerEiNm2 { id: payload.id.clone(), new_ei_nm2: r0.ei_nm2 })
}
