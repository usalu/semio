//! 🔺️ `change-hr-savings-reference-kwh` sparse diff construction — writes only `Din16798Diff.hr_savings_reference_kwh` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_hr_savings_reference_kwh::mutation::ChangeHrSavingsReferenceKwh;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeHrSavingsReferenceKwh, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_hr_savings_reference_kwh.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Heat recovery savings reference must be a finite number, got {}.", payload.new_hr_savings_reference_kwh), Vec::<String>::new());
    }
    if base.hr_savings_reference_kwh == payload.new_hr_savings_reference_kwh {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Heat recovery savings reference is already {}.", payload.new_hr_savings_reference_kwh));
    }
    protocol::MutationOutcome::new(Din16798Diff { hr_savings_reference_kwh: Some(payload.new_hr_savings_reference_kwh.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
