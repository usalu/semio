//! 💥 `change-impact-action-x` diff: patches `x` of the addressed impact action row.

use crate::change_impact_action_x::ChangeImpactActionX;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991ImpactActionDelta, En1991ImpactActionPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeImpactActionX, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_x.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.impact_actions.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Impact action {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.x == payload.new_x {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The impact action already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { impact_actions: En1991ImpactActionDelta::modification(&payload.id, En1991ImpactActionPatch { x: Some(payload.new_x), ..Default::default() }), ..Default::default() })
}
