//! ➖️ `remove-plated-panel` diff — removes the row at the index.

use crate::remove_plated_panel::RemovePlatedPanel;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993PlatedPanelDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &RemovePlatedPanel, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.plated_panels.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("plated-panel index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { plated_panels: En1993PlatedPanelDelta::removal(&base.plated_panels, payload.index), ..Default::default() })
}
