//! 🔺️ `change-delta-tau-stud-mpa` — sparse diff construction.

use super::mutation::ChangeDeltaTauStudMpa;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeDeltaTauStudMpa, _base: &En1994Snapshot) -> En1994Diff {
    En1994Diff { delta_tau_stud_mpa: Some(payload.new_delta_tau_stud_mpa), ..Default::default() }
}
//#endregion 🔖️Diff
