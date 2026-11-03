//! 🎻 `change-prestress-loss` diff: patches `loss_ratio` of the addressed prestress row.

use crate::change_prestress_loss::ChangePrestressLoss;
use semio_s_artifact_norm_en1996_diff::{En1996Diff, En1996PrestressDelta, En1996PrestressPatch, En1996WallDelta, En1996WallPatch};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &ChangePrestressLoss, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if !payload.new_loss_ratio.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.walls.iter().find(|row| row.id == payload.wall_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("wall {} does not exist.", payload.wall_id), [payload.wall_id.clone()]);
    };
    let Some(r1) = r0.prestress.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("prestress {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r1.loss_ratio == payload.new_loss_ratio {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The prestress already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::modification(&payload.wall_id, En1996WallPatch { prestress: En1996PrestressDelta::modification(&payload.id, En1996PrestressPatch { loss_ratio: Some(payload.new_loss_ratio), ..Default::default() }), ..Default::default() }), ..Default::default() })
}
