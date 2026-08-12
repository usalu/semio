//! 🔺️ `remove-selection-constraint` — sparse diff construction; an out-of-range BASE index is a
//! no-op clone (nothing to remove).

use super::mutation::RemoveSelectionConstraint;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &RemoveSelectionConstraint, base: &Iso16757Snapshot) -> Iso16757Diff {
    let mut selection = base.selection.clone();
    if payload.index < selection.constraints.len() {
        selection.constraints.remove(payload.index);
    }
    Iso16757Diff { selection: Some(selection), ..Default::default() }
}
//#endregion 🔖️Diff
