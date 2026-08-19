//! 🔺️ `change-infiltration-allowance-m3-h` sparse diff construction — writes only `Din16798Diff.infiltration_allowance_m3_h` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_infiltration_allowance_m3_h::mutation::ChangeInfiltrationAllowanceM3H;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeInfiltrationAllowanceM3H, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_infiltration_allowance_m3_h.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Infiltration allowance must be a finite number, got {}.", payload.new_infiltration_allowance_m3_h), Vec::<String>::new());
    }
    if base.infiltration_allowance_m3_h == payload.new_infiltration_allowance_m3_h {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Infiltration allowance is already {}.", payload.new_infiltration_allowance_m3_h));
    }
    protocol::MutationOutcome::new(Din16798Diff { infiltration_allowance_m3_h: Some(payload.new_infiltration_allowance_m3_h.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
