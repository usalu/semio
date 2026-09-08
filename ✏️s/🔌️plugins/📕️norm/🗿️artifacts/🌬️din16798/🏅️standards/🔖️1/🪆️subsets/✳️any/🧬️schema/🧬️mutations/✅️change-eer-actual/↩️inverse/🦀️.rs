//! ↩️ `change-eer-actual` inverse — restores the pre-change `eer_actual` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_eer_actual::ChangeEerActual;
use crate::mutations::Din16798Mutation;
use crate::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeEerActual, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeEerActual(ChangeEerActual { new_eer_actual: base.eer_actual })]
}
//#endregion 🔖️Inverse
