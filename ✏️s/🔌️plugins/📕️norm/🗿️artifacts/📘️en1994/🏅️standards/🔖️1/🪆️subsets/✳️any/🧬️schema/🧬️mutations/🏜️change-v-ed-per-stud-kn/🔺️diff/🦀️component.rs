//! 🔺️ `change-v-ed-per-stud-kn` — sparse diff construction.

use super::mutation::ChangeVEdPerStudKn;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeVEdPerStudKn, _base: &En1994Snapshot) -> En1994Diff {
    En1994Diff { v_ed_per_stud_kn: Some(payload.new_v_ed_per_stud_kn), ..Default::default() }
}
//#endregion 🔖️Diff
