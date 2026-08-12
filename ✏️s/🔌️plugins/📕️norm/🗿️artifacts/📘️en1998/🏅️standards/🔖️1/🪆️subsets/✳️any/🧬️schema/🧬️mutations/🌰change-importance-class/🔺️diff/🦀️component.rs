//! 🔺️ `change-importance-class` sparse diff construction — writes only `En1998Diff.importance_class` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_importance_class::mutation::ChangeImportanceClass;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeImportanceClass, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { importance_class: Some(payload.new_importance_class.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
