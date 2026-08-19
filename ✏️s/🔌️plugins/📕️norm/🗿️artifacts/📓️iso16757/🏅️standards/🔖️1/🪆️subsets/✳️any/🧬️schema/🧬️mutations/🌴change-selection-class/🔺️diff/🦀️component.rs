//! 🔺️ `change-selection-class` — sparse diff construction.

use super::mutation::ChangeSelectionClass;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangeSelectionClass, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.selection.class_id == payload.new_class_id {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Selection class is already \"{}\".", payload.new_class_id));
    }
    let mut selection = base.selection.clone();
    selection.class_id = payload.new_class_id.clone();
    protocol::MutationOutcome::new(Iso16757Diff { selection: Some(selection), ..Default::default() })
}
//#endregion 🔖️Diff
