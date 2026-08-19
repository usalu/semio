//! ↩️ `change-span-m` — undo restores BASE's span_m.

use super::mutation::ChangeSpanM;
use crate::artifacts::en1994::{En1994Mutation, En1994Snapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeSpanM, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    vec![En1994Mutation::ChangeSpanM(ChangeSpanM { new_span_m: base.span_m })]
}
//#endregion 🔖️Inverse
