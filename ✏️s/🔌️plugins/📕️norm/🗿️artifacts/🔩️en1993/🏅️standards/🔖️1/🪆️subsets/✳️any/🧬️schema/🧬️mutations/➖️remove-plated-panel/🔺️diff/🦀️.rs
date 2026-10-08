//! ➖️ `remove-plated-panel` diff — removes the row at the index, guarded by the row's own id.

use super::RemovePlatedPanel;
use crate::diff::{En1993Diff, En1993PlatedPanelDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &RemovePlatedPanel, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.plated_panels.get(payload.index) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("plated-panel index {} out of range.", payload.index), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1993Diff { plated_panels: En1993PlatedPanelDelta::removal(&row.id), ..Default::default() })
}
