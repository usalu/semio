//! ↩️ `change-span-m` inverse — restores the pre-change `span_m` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1992::mutations::change_span_m::mutation::ChangeSpanM;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeSpanM, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeSpanM(ChangeSpanM { new_span_m: base.span_m.clone() })]
}
//#endregion 🔖️Inverse
