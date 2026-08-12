//! 🔺️ `change-hoist-class` — sparse diff construction.

use super::mutation::ChangeHoistClass;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeHoistClass, _base: &En1991Snapshot) -> En1991Diff {
    En1991Diff { hoist_class: Some(payload.new_hoist_class.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
