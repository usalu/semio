//! ➕️ `insert-plated-action` diff: inserts the element into the addressed row's list, clamped to its end.

use crate::insert_plated_action::InsertPlatedAction;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993PlatedPanelDelta, En1993PlatedPanelPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &InsertPlatedAction, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.plated_panels.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "plated-action", Vec::<String>::new());
    };
    if row.actions.iter().any(|existing| existing.id == payload.element.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Plated panel action id {} already exists.", payload.element.id), [payload.element.id.clone()]);
    }
    let mut value = row.actions.clone();
    let at = payload.index.unwrap_or(usize::MAX).min(value.len());
    value.insert(at, payload.element.clone());
    protocol::MutationOutcome::new(En1993Diff { plated_panels: En1993PlatedPanelDelta::modification(payload.id.clone(), En1993PlatedPanelPatch { actions: Some(value), ..Default::default() }), ..Default::default() })
}
