//! ✏️ `change-tower-q-nominal` diff: patches `q_nominal` of the addressed tower row.

use crate::change_tower_q_nominal::ChangeTowerQNominal;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998TowerDelta, En1998TowerPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeTowerQNominal, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_q_nominal.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.towers.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("tower {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.q_nominal == payload.new_q_nominal {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The tower already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { towers: En1998TowerDelta::modification(&payload.id, En1998TowerPatch { q_nominal: Some(payload.new_q_nominal), ..Default::default() }), ..Default::default() })
}
