//! ↩️ `change-a-mm2` inverse — restores the pre-change `a_mm2` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1999::mutations::change_a_mm2::mutation::ChangeAMm2;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeAMm2, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeAMm2(ChangeAMm2 { new_a_mm2: base.a_mm2.clone() })]
}
//#endregion 🔖️Inverse
