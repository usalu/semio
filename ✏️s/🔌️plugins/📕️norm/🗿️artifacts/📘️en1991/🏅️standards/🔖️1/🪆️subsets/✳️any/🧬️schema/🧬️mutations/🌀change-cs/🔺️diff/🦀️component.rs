//! 🔺️ `change-cs` — sparse diff construction.

use super::mutation::ChangeCS;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeCS, _base: &En1991Snapshot) -> En1991Diff {
    En1991Diff { c_s: Some(payload.new_c_s.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
