//! 🔺️ `change-qc-kwh` sparse diff construction — writes only `Din16798Diff.q_c_kwh` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_q_c_kwh::mutation::ChangeQCKwh;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeQCKwh, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_q_c_kwh.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Cooling energy demand must be a finite number, got {}.", payload.new_q_c_kwh), Vec::<String>::new());
    }
    if base.q_c_kwh == payload.new_q_c_kwh {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Cooling energy demand is already {}.", payload.new_q_c_kwh));
    }
    protocol::MutationOutcome::new(Din16798Diff { q_c_kwh: Some(payload.new_q_c_kwh.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
