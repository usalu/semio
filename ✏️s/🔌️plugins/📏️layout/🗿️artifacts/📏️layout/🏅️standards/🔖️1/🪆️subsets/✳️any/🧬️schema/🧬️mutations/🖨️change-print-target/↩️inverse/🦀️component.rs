//! ↩ Inverse constructor for `change-print-target` — reconstructed from captured BASE state.

use super::mutation::ChangePrintTarget;
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::LayoutSnapshot;

//#region 🖨️ChangePrintTarget
pub async fn inverse_change_print_target(_payload: &ChangePrintTarget, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    vec![LayoutMutation::ChangePrintTarget(ChangePrintTarget { new_print_target: base.print_target.clone() })]
}
//#endregion 🖨️ChangePrintTarget
