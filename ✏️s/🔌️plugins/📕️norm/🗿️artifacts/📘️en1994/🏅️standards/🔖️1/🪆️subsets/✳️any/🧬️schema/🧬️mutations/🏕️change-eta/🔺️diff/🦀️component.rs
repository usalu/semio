//! 🔺️ `change-eta` — sparse diff construction.

use super::mutation::ChangeEta;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeEta, _base: &En1994Snapshot) -> En1994Diff {
    En1994Diff { eta: Some(payload.new_eta), ..Default::default() }
}
//#endregion 🔖️Diff
