//! ↩️ `change-t-int-c` — undo restores BASE's `t_int_c`.

use super::ChangeTIntC;
use crate::artifacts::din4108::{Din4108Mutation, Din4108Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeTIntC, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    vec![Din4108Mutation::ChangeTIntC(ChangeTIntC { new_t_int_c: base.t_int_c })]
}
//#endregion 🔖️Inverse
