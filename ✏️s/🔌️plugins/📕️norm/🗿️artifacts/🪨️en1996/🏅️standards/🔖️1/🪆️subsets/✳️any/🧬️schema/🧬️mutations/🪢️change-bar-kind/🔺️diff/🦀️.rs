//! 🪢️ `change-bar-kind` diff: patches `kind` of the addressed bar row.

use crate::change_bar_kind::ChangeBarKind;
use semio_s_artifact_norm_en1996_diff::{En1996BarDelta, En1996BarPatch, En1996Diff, En1996WallDelta, En1996WallPatch};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &ChangeBarKind, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    let Some(r0) = base.walls.iter().find(|row| row.id == payload.wall_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("wall {} does not exist.", payload.wall_id), [payload.wall_id.clone()]);
    };
    let Some(r1) = r0.bars.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("bar {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r1.kind == payload.new_kind {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The bar already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::modification(&payload.wall_id, En1996WallPatch { bars: En1996BarDelta::modification(&payload.id, En1996BarPatch { kind: Some(payload.new_kind.clone()), ..Default::default() }), ..Default::default() }), ..Default::default() })
}
