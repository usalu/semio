//! 🔺️ `change-dhw-delivery-c` sparse diff construction — writes only `Din16798Diff.dhw_delivery_c` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_dhw_delivery_c::mutation::ChangeDhwDeliveryC;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeDhwDeliveryC, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_dhw_delivery_c.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("DHW delivery temperature must be a finite number, got {}.", payload.new_dhw_delivery_c), Vec::<String>::new());
    }
    if base.dhw_delivery_c == payload.new_dhw_delivery_c {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("DHW delivery temperature is already {}.", payload.new_dhw_delivery_c));
    }
    protocol::MutationOutcome::new(Din16798Diff { dhw_delivery_c: Some(payload.new_dhw_delivery_c.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
