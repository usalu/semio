//! 🔺️ `change-area-m2` — sparse diff construction.

use super::mutation::ChangeAreaM2;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeAreaM2, _base: &En1991Snapshot) -> En1991Diff {
    En1991Diff { area_m2: Some(payload.new_area_m2.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
