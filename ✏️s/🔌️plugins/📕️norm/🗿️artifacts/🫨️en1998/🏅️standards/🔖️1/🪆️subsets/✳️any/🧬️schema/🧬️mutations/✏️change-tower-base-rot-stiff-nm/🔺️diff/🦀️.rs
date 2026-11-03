//! ✏️ `change-tower-base-rot-stiff-nm` diff: patches `base_rotational_stiffness_nm` of the addressed tower row.

use crate::change_tower_base_rot_stiff_nm::ChangeTowerBaseRotStiffNm;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998TowerDelta, En1998TowerPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeTowerBaseRotStiffNm, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_base_rotational_stiffness_nm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.towers.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("tower {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.base_rotational_stiffness_nm == payload.new_base_rotational_stiffness_nm {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The tower already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { towers: En1998TowerDelta::modification(&payload.id, En1998TowerPatch { base_rotational_stiffness_nm: Some(payload.new_base_rotational_stiffness_nm), ..Default::default() }), ..Default::default() })
}
