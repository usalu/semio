//! 🔺️ `change-v-ed-kn` — sparse diff construction.

use super::mutation::ChangeVEdKn;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeVEdKn, _base: &En1994Snapshot) -> En1994Diff {
    En1994Diff { v_ed_kn: Some(payload.new_v_ed_kn), ..Default::default() }
}
//#endregion 🔖️Diff
