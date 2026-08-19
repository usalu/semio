//! ↩️ `change-v-ed-per-stud-kn` — undo restores BASE's v_ed_per_stud_kn.

use super::mutation::ChangeVEdPerStudKn;
use crate::artifacts::en1994::{En1994Mutation, En1994Snapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeVEdPerStudKn, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    vec![En1994Mutation::ChangeVEdPerStudKn(ChangeVEdPerStudKn { new_v_ed_per_stud_kn: base.v_ed_per_stud_kn })]
}
//#endregion 🔖️Inverse
