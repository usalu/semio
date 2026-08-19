//! 🔺️ `add-selection-constraint` — sparse diff construction.

use super::mutation::AddSelectionConstraint;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &AddSelectionConstraint, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.selection.constraints.contains(&payload.constraint) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Selection constraint on \"{}\" already exists.", payload.constraint.property_id));
    }
    let mut selection = base.selection.clone();
    selection.constraints.push(payload.constraint.clone());
    protocol::MutationOutcome::new(Iso16757Diff { selection: Some(selection), ..Default::default() })
}
//#endregion 🔖️Diff
