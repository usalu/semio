//! 🔺️ `add-selection-constraint` — sparse diff construction.

use super::mutation::AddSelectionConstraint;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &AddSelectionConstraint, base: &Iso16757Snapshot) -> Iso16757Diff {
    let mut selection = base.selection.clone();
    selection.constraints.push(payload.constraint.clone());
    Iso16757Diff { selection: Some(selection), ..Default::default() }
}
//#endregion 🔖️Diff
