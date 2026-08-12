//! 🔺 Diff constructor for `change-print-target`.

use super::mutation::ChangePrintTarget;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};

//#region 🖨️ChangePrintTarget
pub fn diff_change_print_target(payload: &ChangePrintTarget, _base: &LayoutSnapshot) -> LayoutDiff {
    LayoutDiff { print_target: Some(payload.new_print_target.clone()), ..Default::default() }
}
//#endregion 🖨️ChangePrintTarget
