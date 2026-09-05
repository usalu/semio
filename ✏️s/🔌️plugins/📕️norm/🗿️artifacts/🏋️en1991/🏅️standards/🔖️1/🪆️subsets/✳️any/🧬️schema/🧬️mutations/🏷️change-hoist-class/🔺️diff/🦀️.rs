//! 🔺️ `change-hoist-class` — sparse diff construction.

use super::ChangeHoistClass;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeHoistClass, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.hoist_class == payload.new_hoist_class {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Hoist class already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { hoist_class: Some(payload.new_hoist_class.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
