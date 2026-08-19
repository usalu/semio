//! 🔺️ `change-data-center-supply-c` sparse diff construction — writes only `Din16798Diff.data_center_supply_c` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_data_center_supply_c::mutation::ChangeDataCenterSupplyC;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeDataCenterSupplyC, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_data_center_supply_c.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Data center supply temperature must be a finite number, got {}.", payload.new_data_center_supply_c), Vec::<String>::new());
    }
    if base.data_center_supply_c == payload.new_data_center_supply_c {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Data center supply temperature is already {}.", payload.new_data_center_supply_c));
    }
    protocol::MutationOutcome::new(Din16798Diff { data_center_supply_c: Some(payload.new_data_center_supply_c.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
