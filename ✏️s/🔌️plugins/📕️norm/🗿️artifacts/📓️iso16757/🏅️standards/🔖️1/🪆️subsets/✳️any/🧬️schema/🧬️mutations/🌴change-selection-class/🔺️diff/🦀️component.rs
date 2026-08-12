//! 🔺️ `change-selection-class` — sparse diff construction.

use super::mutation::ChangeSelectionClass;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeSelectionClass, base: &Iso16757Snapshot) -> Iso16757Diff {
    let mut selection = base.selection.clone();
    selection.class_id = payload.new_class_id.clone();
    Iso16757Diff { selection: Some(selection), ..Default::default() }
}
//#endregion 🔖️Diff
