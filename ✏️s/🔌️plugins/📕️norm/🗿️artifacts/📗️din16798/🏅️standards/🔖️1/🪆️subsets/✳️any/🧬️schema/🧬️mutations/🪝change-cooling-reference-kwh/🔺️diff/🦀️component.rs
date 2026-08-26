//! 🔺️ `change-cooling-reference-kwh` sparse diff construction — writes only `Din16798Diff.cooling_reference_kwh` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_cooling_reference_kwh::mutation::ChangeCoolingReferenceKwh;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeCoolingReferenceKwh, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_cooling_reference_kwh.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Cooling energy reference must be a finite number, got {}.", payload.new_cooling_reference_kwh), Vec::<String>::new());
    }
    if base.cooling_reference_kwh == payload.new_cooling_reference_kwh {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Cooling energy reference is already {}.", payload.new_cooling_reference_kwh));
    }
    protocol::MutationOutcome::new(Din16798Diff { cooling_reference_kwh: Some(payload.new_cooling_reference_kwh.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
