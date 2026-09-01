//! ↩️ `change-delta-tau-stud-mpa` — undo restores BASE's delta_tau_stud_mpa.

use super::ChangeDeltaTauStudMpa;
use crate::artifacts::en1994::{En1994Mutation, En1994Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeDeltaTauStudMpa, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    vec![En1994Mutation::ChangeDeltaTauStudMpa(ChangeDeltaTauStudMpa { new_delta_tau_stud_mpa: base.delta_tau_stud_mpa })]
}
//#endregion 🔖️Inverse
