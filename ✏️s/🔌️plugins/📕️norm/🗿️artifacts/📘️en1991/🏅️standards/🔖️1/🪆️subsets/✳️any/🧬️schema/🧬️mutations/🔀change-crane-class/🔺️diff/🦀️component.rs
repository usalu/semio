//! 🔺️ `change-crane-class` — sparse diff construction.

use super::mutation::ChangeCraneClass;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeCraneClass, _base: &En1991Snapshot) -> En1991Diff {
    En1991Diff { crane_class: Some(payload.new_crane_class.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
