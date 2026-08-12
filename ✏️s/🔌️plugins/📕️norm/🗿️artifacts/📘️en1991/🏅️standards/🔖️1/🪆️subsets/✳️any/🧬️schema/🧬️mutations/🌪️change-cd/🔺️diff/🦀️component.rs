//! 🔺️ `change-cd` — sparse diff construction.

use super::mutation::ChangeCD;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeCD, _base: &En1991Snapshot) -> En1991Diff {
    En1991Diff { c_d: Some(payload.new_c_d.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
