//! ↩️ `change-tower-m-rd-nm` inverse: restores the previous `m_rd_nm`.

use crate::change_tower_m_rd_nm::ChangeTowerMRdNm;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeTowerMRdNm, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tower_m_rd_nm::ChangeTowerMRdNm> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTowerMRdNm, base: &En1998Snapshot) -> Option<ChangeTowerMRdNm> {
    let r0 = base.towers.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTowerMRdNm { id: payload.id.clone(), new_m_rd_nm: r0.m_rd_nm })
}
