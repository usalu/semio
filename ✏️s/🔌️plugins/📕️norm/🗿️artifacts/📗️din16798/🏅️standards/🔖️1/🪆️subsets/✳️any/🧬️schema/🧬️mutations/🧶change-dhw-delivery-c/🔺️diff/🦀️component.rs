//! 🔺️ `change-dhw-delivery-c` sparse diff construction — writes only `Din16798Diff.dhw_delivery_c` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_dhw_delivery_c::mutation::ChangeDhwDeliveryC;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeDhwDeliveryC, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { dhw_delivery_c: Some(payload.new_dhw_delivery_c.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
