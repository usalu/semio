//! 🔺️ `remove-selection-constraint` — sparse diff construction; an out-of-range BASE index is
//! `mutation.target-missing`.

use super::mutation::RemoveSelectionConstraint;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &RemoveSelectionConstraint, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if payload.index >= base.selection.constraints.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Selection constraint #{} does not exist.", payload.index), [payload.index.to_string()]);
    }
    let mut selection = base.selection.clone();
    selection.constraints.remove(payload.index);
    protocol::MutationOutcome::new(Iso16757Diff { selection: Some(selection), ..Default::default() })
}
//#endregion 🔖️Diff
