//! 📐️ `change-tributary-area` diff: patches `tributary_area_m2` of the addressed load case row.

use crate::change_tributary_area::ChangeTributaryArea;
use semio_s_artifact_norm_en1996_diff::{En1996Diff, En1996LoadCaseDelta, En1996LoadCasePatch, En1996WallDelta, En1996WallPatch};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &ChangeTributaryArea, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if !payload.new_tributary_area_m2.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.walls.iter().find(|row| row.id == payload.wall_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("wall {} does not exist.", payload.wall_id), [payload.wall_id.clone()]);
    };
    let Some(r1) = r0.load_cases.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("load case {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r1.tributary_area_m2 == payload.new_tributary_area_m2 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The load case already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::modification(&payload.wall_id, En1996WallPatch { load_cases: En1996LoadCaseDelta::modification(&payload.id, En1996LoadCasePatch { tributary_area_m2: Some(payload.new_tributary_area_m2), ..Default::default() }), ..Default::default() }), ..Default::default() })
}
