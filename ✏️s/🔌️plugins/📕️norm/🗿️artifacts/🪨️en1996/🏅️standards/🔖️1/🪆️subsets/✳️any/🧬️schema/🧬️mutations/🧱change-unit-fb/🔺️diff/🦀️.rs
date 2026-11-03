//! 🧱 `change-unit-fb` diff: patches `f_b_pa` of the addressed wall row.

use crate::change_unit_fb::ChangeUnitFb;
use semio_s_artifact_norm_en1996_diff::{En1996Diff, En1996WallDelta, En1996WallPatch};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &ChangeUnitFb, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if !payload.new_f_b_pa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.walls.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("wall {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.f_b_pa == payload.new_f_b_pa {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The wall already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::modification(&payload.id, En1996WallPatch { f_b_pa: Some(payload.new_f_b_pa), ..Default::default() }), ..Default::default() })
}
