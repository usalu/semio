//! ➕️ `insert-tension-action` diff: inserts the element into the addressed row's list, clamped to its end.

use crate::insert_tension_action::InsertTensionAction;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993TensionComponentDelta, En1993TensionComponentPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &InsertTensionAction, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.tension_components.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "tension-action", Vec::<String>::new());
    };
    if row.actions.iter().any(|existing| existing.id == payload.element.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Tension component action id {} already exists.", payload.element.id), [payload.element.id.clone()]);
    }
    let mut value = row.actions.clone();
    let at = payload.index.unwrap_or(usize::MAX).min(value.len());
    value.insert(at, payload.element.clone());
    protocol::MutationOutcome::new(En1993Diff { tension_components: En1993TensionComponentDelta::modification(payload.id.clone(), En1993TensionComponentPatch { actions: Some(value), ..Default::default() }), ..Default::default() })
}
