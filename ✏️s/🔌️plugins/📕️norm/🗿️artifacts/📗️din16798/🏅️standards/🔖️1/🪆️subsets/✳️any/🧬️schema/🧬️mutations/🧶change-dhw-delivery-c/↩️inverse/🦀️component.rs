//! ↩️ `change-dhw-delivery-c` inverse — restores the pre-change `dhw_delivery_c` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_dhw_delivery_c::mutation::ChangeDhwDeliveryC;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeDhwDeliveryC, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeDhwDeliveryC(ChangeDhwDeliveryC { new_dhw_delivery_c: base.dhw_delivery_c.clone() })]
}
//#endregion 🔖️Inverse
