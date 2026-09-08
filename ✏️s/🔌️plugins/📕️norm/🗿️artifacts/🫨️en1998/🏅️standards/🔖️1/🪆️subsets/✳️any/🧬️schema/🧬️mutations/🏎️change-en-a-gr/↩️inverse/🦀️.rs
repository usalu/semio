//! ↩️ `change-en-a-gr` inverse — restores the pre-change `en_a_gr` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_en_a_gr::ChangeEnAGr;
use crate::mutations::En1998Mutation;
use crate::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeEnAGr, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeEnAGr(ChangeEnAGr { new_en_a_gr: base.en_a_gr })]
}
//#endregion 🔖️Inverse
