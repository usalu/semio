//! 🔺️ `change-construction-activity` — sparse diff construction.

use super::mutation::ChangeConstructionActivity;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeConstructionActivity, _base: &En1991Snapshot) -> En1991Diff {
    En1991Diff { construction_activity: Some(payload.new_construction_activity.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
