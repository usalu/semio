use super::InsertPlatedPanel;
use crate::diff::En1993PlatedList;
use crate::{En1993Diff, En1993Snapshot};
pub fn diff(payload: &InsertPlatedPanel, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.plated_panels.iter().any(|existing| existing.id == payload.plated_panel.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Plated panel id {} already exists.", payload.plated_panel.id), [payload.plated_panel.id.clone()]);
    }
    let mut values = base.plated_panels.clone();
    let at = payload.index.min(values.len());
    values.insert(at, payload.plated_panel.clone());
    protocol::MutationOutcome::new(En1993Diff { plated_panels: Some(En1993PlatedList { values }), ..Default::default() })
}
