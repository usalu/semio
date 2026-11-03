//! 🧷 `change-bracing-system` diff: patches `bracing_system` of the addressed wall row.

use crate::change_bracing_system::ChangeBracingSystem;
use semio_s_artifact_norm_en1996_diff::{En1996Diff, En1996WallDelta, En1996WallPatch};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &ChangeBracingSystem, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    let Some(r0) = base.walls.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Wall {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.bracing_system == payload.new_bracing_system {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The wall already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::modification(&payload.id, En1996WallPatch { bracing_system: Some(payload.new_bracing_system), ..Default::default() }), ..Default::default() })
}
