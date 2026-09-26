//! Inverse for `change-tower-m-rd-nm`.
use super::ChangeTowerMRdNm;
use crate::{En1998Mutation, En1998Snapshot};

pub fn inverse(payload: &ChangeTowerMRdNm, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    match base.towers.get(payload.index) {
        Some(t) => vec![En1998Mutation::ChangeTowerMRdNm(ChangeTowerMRdNm { index: payload.index, new_m_rd_nm: t.m_rd_nm })],
        None => Vec::new(),
    }
}
