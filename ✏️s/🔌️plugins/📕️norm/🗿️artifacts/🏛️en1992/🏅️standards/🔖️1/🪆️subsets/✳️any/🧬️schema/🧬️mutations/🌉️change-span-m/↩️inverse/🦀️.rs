//! ↩️ `change-span-m` inverse — restores the pre-change `span_m` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_span_m::ChangeSpanM;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeSpanM, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeSpanM(ChangeSpanM { new_span_m: base.span_m })]
}
//#endregion 🔖️Inverse
