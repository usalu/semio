//! ➕️ `insert-plated-panel` diff — inserts the row at its position, clamped to the end of the collection.

use super::InsertPlatedPanel;
use crate::diff::En1993RowEdit as _;
use crate::diff::{En1993Diff, En1993PlatedPanelEdit};
use crate::En1993Snapshot;

pub fn diff(payload: &InsertPlatedPanel, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.plated_panels.iter().any(|existing| existing.id == payload.plated_panel.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Plated panel id {} already exists.", payload.plated_panel.id), [payload.plated_panel.id.clone()]);
    }
    let index = payload.index.min(base.plated_panels.len());
    protocol::MutationOutcome::new(En1993Diff { plated_panels: vec![En1993PlatedPanelEdit::insert(index, payload.plated_panel.clone())], ..Default::default() })
}
