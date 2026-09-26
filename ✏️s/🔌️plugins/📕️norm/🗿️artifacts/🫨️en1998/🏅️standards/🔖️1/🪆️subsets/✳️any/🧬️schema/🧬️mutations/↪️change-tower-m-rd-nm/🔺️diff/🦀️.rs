//! Diff for `change-tower-m-rd-nm`.
use super::ChangeTowerMRdNm;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &ChangeTowerMRdNm, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let mut towers = base.towers.clone();
    let Some(t) = towers.get_mut(payload.index) else { return protocol::MutationOutcome::error("mutation.target-missing", "tower", Vec::<String>::new()); };
    t.m_rd_nm = payload.new_m_rd_nm;
    protocol::MutationOutcome::new(En1998Diff { towers: Some(towers), ..Default::default() })
}
