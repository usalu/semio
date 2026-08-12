//! 🔺️ `change-n-cycles-stud` — sparse diff construction.

use super::mutation::ChangeNCyclesStud;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeNCyclesStud, _base: &En1994Snapshot) -> En1994Diff {
    En1994Diff { n_cycles_stud: Some(payload.new_n_cycles_stud), ..Default::default() }
}
//#endregion 🔖️Diff
