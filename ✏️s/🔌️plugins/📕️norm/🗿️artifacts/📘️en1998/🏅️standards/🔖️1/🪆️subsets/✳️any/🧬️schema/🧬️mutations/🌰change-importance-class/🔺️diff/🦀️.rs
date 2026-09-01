//! 🔺️ `change-importance-class` sparse diff construction — writes only `En1998Diff.importance_class` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_importance_class::ChangeImportanceClass;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeImportanceClass, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.importance_class == payload.new_importance_class {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Importance class is already \"{}\".", payload.new_importance_class));
    }
    protocol::MutationOutcome::new(En1998Diff { importance_class: Some(payload.new_importance_class.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
