//! 🔺️ `change-construction-activity` — sparse diff construction.

use super::ChangeConstructionActivity;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeConstructionActivity, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.construction_activity == payload.new_construction_activity {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Construction activity already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { construction_activity: Some(payload.new_construction_activity.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
