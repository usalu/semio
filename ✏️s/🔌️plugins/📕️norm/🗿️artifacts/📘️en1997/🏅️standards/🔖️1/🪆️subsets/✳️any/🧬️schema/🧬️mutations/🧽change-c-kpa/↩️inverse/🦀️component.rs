//! ↩️ `change-c-kpa` inverse — restores the pre-change `c_kpa` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1997::mutations::change_c_kpa::mutation::ChangeCKpa;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeCKpa, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    vec![En1997Mutation::ChangeCKpa(ChangeCKpa { new_c_kpa: base.c_kpa.clone() })]
}
//#endregion 🔖️Inverse
