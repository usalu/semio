//! ↩️ `change-bedrooms` inverse — restores the pre-change `bedrooms` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_bedrooms::ChangeBedrooms;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeBedrooms, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeBedrooms(ChangeBedrooms { new_bedrooms: base.bedrooms.clone() })]
}
//#endregion 🔖️Inverse
