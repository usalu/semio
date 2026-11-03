//! 🏋️ `change-anchor-action-point-force` diff: patches `point_force` of the addressed anchor action row.

use crate::change_anchor_action_point_force::ChangeAnchorActionPointForce;
use semio_s_artifact_norm_en1992_diff::{En1992AnchorsActionsPatch, En1992AnchorsActionsRows, En1992AnchorsPatch, En1992AnchorsRows, En1992Diff};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &ChangeAnchorActionPointForce, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_point_force.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.anchors.iter().find(|row| row.id == payload.anchor_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Anchor {} does not exist.", payload.anchor_id), [payload.anchor_id.clone()]);
    };
    let Some(r1) = r0.actions.iter().find(|row| row.id == payload.action_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Anchor action {} does not exist.", payload.action_id), [payload.action_id.clone()]);
    };
    if r1.point_force == payload.new_point_force {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The anchor action already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { anchors: Some(En1992AnchorsRows::modification(&payload.anchor_id, En1992AnchorsPatch { actions: Some(En1992AnchorsActionsRows::modification(&payload.action_id, En1992AnchorsActionsPatch { point_force: Some(payload.new_point_force), ..Default::default() })), ..Default::default() })), ..Default::default() })
}
