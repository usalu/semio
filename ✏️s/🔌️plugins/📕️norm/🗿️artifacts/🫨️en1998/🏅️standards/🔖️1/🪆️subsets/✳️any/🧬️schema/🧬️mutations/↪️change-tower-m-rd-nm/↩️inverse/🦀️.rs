//! Inverse for `change-tower-m-rd-nm`.
use crate::change_tower_m_rd_nm::ChangeTowerMRdNm;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeTowerMRdNm, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tower_m_rd_nm::ChangeTowerMRdNm> {
    Ok((|| {
    match base.towers.get(payload.index) {
        Some(t) => vec![Op::from(ChangeTowerMRdNm { index: payload.index, new_m_rd_nm: t.m_rd_nm })],
        None => Vec::new(),
    }

    })())
}
