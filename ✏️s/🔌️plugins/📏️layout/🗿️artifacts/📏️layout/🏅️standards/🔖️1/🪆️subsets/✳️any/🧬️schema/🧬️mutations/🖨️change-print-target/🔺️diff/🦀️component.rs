//! 🔺 Diff constructor for `change-print-target`.

use super::mutation::ChangePrintTarget;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};

//#region 🖨️ChangePrintTarget
pub fn diff_change_print_target(payload: &ChangePrintTarget, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if base.print_target == payload.new_print_target {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Print target is already set to that value.");
    }
    protocol::MutationOutcome::new(LayoutDiff { print_target: Some(payload.new_print_target.clone()), ..Default::default() })
}
//#endregion 🖨️ChangePrintTarget
