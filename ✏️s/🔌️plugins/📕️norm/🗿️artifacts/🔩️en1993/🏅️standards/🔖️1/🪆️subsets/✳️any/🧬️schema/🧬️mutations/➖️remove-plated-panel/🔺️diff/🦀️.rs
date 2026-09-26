use super::RemovePlatedPanel;
use crate::diff::En1993PlatedList;
use crate::{En1993Diff, En1993Snapshot};
pub fn diff(payload: &RemovePlatedPanel, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.plated_panels.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("plated-panel index {} out of range.", payload.index), Vec::<String>::new());
    }
    let mut values = base.plated_panels.clone();
    values.remove(payload.index);
    protocol::MutationOutcome::new(En1993Diff { plated_panels: Some(En1993PlatedList { values }), ..Default::default() })
}
