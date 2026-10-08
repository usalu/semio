//! 🧱️ `update-plated-inputs` diff — upserts the row by id: a known id is replaced in place, an unknown id is appended.

use super::UpdatePlatedInputs;
use crate::diff::En1993RowEdit as _;
use crate::diff::{En1993Diff, En1993PlatedPanelEdit};
use crate::En1993Snapshot;

pub fn diff(payload: &UpdatePlatedInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let edit = match base.plated_panels.iter().position(|row| row.id == payload.plated_panel.id) {
        Some(index) if base.plated_panels[index] == payload.plated_panel => return protocol::MutationOutcome::empty().warning("mutation.no-op", "Entity already has this value."),
        Some(index) => En1993PlatedPanelEdit::replace(index, payload.plated_panel.id.clone(), payload.plated_panel.clone()),
        None => En1993PlatedPanelEdit::insert(base.plated_panels.len(), payload.plated_panel.clone()),
    };
    protocol::MutationOutcome::new(En1993Diff { plated_panels: vec![edit], ..Default::default() })
}
