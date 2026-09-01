//! ↩️ `change-n-cycles-stud` — undo restores BASE's n_cycles_stud.

use super::ChangeNCyclesStud;
use crate::artifacts::en1994::{En1994Mutation, En1994Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeNCyclesStud, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    vec![En1994Mutation::ChangeNCyclesStud(ChangeNCyclesStud { new_n_cycles_stud: base.n_cycles_stud })]
}
//#endregion 🔖️Inverse
