//! ✏️ `change-system-alpha-u-over-alpha1` diff: patches `alpha_u_over_alpha_1` of the addressed system row.

use crate::change_system_alpha_u_over_alpha_1::ChangeSystemAlphaUOverAlpha1;
use semio_s_artifact_norm_en1998_diff::{En1998BuildingDelta, En1998BuildingPatch, En1998Diff, En1998SystemDelta, En1998SystemPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeSystemAlphaUOverAlpha1, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_alpha_u_over_alpha_1.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.buildings.iter().find(|row| row.id == payload.building_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("building {} does not exist.", payload.building_id), [payload.building_id.clone()]);
    };
    let Some(r1) = r0.systems.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("system {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r1.alpha_u_over_alpha_1 == payload.new_alpha_u_over_alpha_1 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The system already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { buildings: En1998BuildingDelta::modification(&payload.building_id, En1998BuildingPatch { systems: En1998SystemDelta::modification(&payload.id, En1998SystemPatch { alpha_u_over_alpha_1: Some(payload.new_alpha_u_over_alpha_1), ..Default::default() }), ..Default::default() }), ..Default::default() })
}
