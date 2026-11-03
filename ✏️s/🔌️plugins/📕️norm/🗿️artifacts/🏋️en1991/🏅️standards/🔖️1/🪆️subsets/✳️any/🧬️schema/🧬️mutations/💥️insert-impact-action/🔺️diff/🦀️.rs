//! 💥 `insert-impact-action` diff: inserts the impact action row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_impact_action::InsertImpactAction;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991ImpactActionDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &InsertImpactAction, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.impact_actions.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Impact action {key} already exists."), [key]);
    }
    if payload.index > base.impact_actions.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.impact_actions.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1991Diff { impact_actions: En1991ImpactActionDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
