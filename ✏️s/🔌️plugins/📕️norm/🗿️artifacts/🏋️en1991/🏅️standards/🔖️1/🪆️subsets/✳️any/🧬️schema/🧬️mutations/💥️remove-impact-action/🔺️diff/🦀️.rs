//! 💥 `remove-impact-action` diff: removes the addressed impact action row.

use crate::remove_impact_action::RemoveImpactAction;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991ImpactActionDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &RemoveImpactAction, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(index) = base.impact_actions.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Impact action {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1991Diff { impact_actions: En1991ImpactActionDelta::removal(&base.impact_actions, index), ..Default::default() })
}
