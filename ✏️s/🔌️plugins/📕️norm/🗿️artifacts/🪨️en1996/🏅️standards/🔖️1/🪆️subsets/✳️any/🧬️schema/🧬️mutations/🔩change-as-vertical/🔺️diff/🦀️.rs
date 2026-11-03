//! 🔩 `change-as-vertical` diff: patches `as_vertical_m2` of the addressed wall row.

use crate::change_as_vertical::ChangeAsVertical;
use semio_s_artifact_norm_en1996_diff::{En1996Diff, En1996WallDelta, En1996WallPatch};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &ChangeAsVertical, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if !payload.new_as_vertical_m2.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.walls.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("wall {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.as_vertical_m2 == payload.new_as_vertical_m2 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The wall already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::modification(&payload.id, En1996WallPatch { as_vertical_m2: Some(payload.new_as_vertical_m2), ..Default::default() }), ..Default::default() })
}
