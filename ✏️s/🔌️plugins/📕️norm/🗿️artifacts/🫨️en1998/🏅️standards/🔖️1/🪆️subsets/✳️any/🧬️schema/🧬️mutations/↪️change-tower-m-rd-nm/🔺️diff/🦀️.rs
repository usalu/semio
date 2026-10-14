//! ↪️ `change-tower-m-rd-nm` diff — patches the one field of the row at the index; a missing row is a `mutation.target-missing`.

use crate::change_tower_m_rd_nm::ChangeTowerMRdNm;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998TowerDelta, En1998TowerPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeTowerMRdNm, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(row) = base.towers.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "tower", Vec::<String>::new());
    };
    let patch = En1998TowerPatch { m_rd_nm: Some(payload.new_m_rd_nm), ..Default::default() };
    protocol::MutationOutcome::new(En1998Diff { towers: En1998TowerDelta::modification(&row.id, patch), ..Default::default() })
}
