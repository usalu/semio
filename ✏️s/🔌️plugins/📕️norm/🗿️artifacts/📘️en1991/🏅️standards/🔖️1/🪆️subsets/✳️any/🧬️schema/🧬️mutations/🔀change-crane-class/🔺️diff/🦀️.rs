//! 🔺️ `change-crane-class` — sparse diff construction.

use super::ChangeCraneClass;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeCraneClass, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.crane_class == payload.new_crane_class {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Crane class already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { crane_class: Some(payload.new_crane_class.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
