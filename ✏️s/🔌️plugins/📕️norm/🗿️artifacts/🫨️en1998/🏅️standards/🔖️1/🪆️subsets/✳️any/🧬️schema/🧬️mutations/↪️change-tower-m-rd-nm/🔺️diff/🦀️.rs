//! ↪️ `change-tower-m-rd-nm` diff: patches `m_rd_nm` of the addressed tower row.

use crate::change_tower_m_rd_nm::ChangeTowerMRdNm;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998TowerDelta, En1998TowerPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeTowerMRdNm, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_m_rd_nm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.towers.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("tower {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.m_rd_nm == payload.new_m_rd_nm {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The tower already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { towers: En1998TowerDelta::modification(&payload.id, En1998TowerPatch { m_rd_nm: Some(payload.new_m_rd_nm), ..Default::default() }), ..Default::default() })
}
