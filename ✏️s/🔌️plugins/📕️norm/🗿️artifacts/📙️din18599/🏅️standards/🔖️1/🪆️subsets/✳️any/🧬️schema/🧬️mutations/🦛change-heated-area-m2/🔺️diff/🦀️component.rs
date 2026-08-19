//! 🔺️ `change-heated-area-m2` sparse diff construction — writes only `Din18599Diff.heated_area_m2` from the payload.

use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::change_heated_area_m2::mutation::ChangeHeatedAreaM2;
use crate::artifacts::din18599::Din18599Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeHeatedAreaM2, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    if !payload.new_heated_area_m2.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Heated area m2 must be a finite number.", Vec::<String>::new());
    }
    if base.heated_area_m2 == payload.new_heated_area_m2 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Heated area m2 already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff { heated_area_m2: Some(payload.new_heated_area_m2.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
