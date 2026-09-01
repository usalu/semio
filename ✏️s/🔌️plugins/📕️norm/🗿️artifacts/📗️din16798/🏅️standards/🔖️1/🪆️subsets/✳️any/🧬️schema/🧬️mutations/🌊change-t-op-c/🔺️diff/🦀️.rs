//! 🔺️ `change-t-op-c` sparse diff construction — writes only `Din16798Diff.t_op_c` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_t_op_c::ChangeTOpC;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeTOpC, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_t_op_c.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Operative temperature must be a finite number, got {}.", payload.new_t_op_c), Vec::<String>::new());
    }
    if base.t_op_c == payload.new_t_op_c {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Operative temperature is already {}.", payload.new_t_op_c));
    }
    protocol::MutationOutcome::new(Din16798Diff { t_op_c: Some(payload.new_t_op_c.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
