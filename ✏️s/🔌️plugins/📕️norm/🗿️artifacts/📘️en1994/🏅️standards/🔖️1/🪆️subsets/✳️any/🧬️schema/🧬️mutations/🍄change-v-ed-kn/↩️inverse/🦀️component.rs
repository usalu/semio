//! ↩️ `change-v-ed-kn` — undo restores BASE's v_ed_kn.

use super::mutation::ChangeVEdKn;
use crate::artifacts::en1994::{En1994Mutation, En1994Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeVEdKn, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    vec![En1994Mutation::ChangeVEdKn(ChangeVEdKn { new_v_ed_kn: base.v_ed_kn })]
}
//#endregion 🔖️Inverse
