//! 🔺️ `change-e-cm-mpa` — sparse diff construction.

use super::mutation::ChangeECmMpa;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeECmMpa, _base: &En1994Snapshot) -> En1994Diff {
    En1994Diff { e_cm_mpa: Some(payload.new_e_cm_mpa), ..Default::default() }
}
//#endregion 🔖️Diff
