//! 🔺️ `change-fan-t-run-h` sparse diff construction — writes only `Din16798Diff.fan_t_run_h` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_fan_t_run_h::mutation::ChangeFanTRunH;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeFanTRunH, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_fan_t_run_h.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Fan running time must be a finite number, got {}.", payload.new_fan_t_run_h), Vec::<String>::new());
    }
    if base.fan_t_run_h == payload.new_fan_t_run_h {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Fan running time is already {}.", payload.new_fan_t_run_h));
    }
    protocol::MutationOutcome::new(Din16798Diff { fan_t_run_h: Some(payload.new_fan_t_run_h.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
