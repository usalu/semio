//! ↩️ `change-pile-base-area-m2` inverse — restores the pre-change `pile_base_area_m2` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1997::mutations::change_pile_base_area_m2::mutation::ChangePileBaseAreaM2;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangePileBaseAreaM2, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    vec![En1997Mutation::ChangePileBaseAreaM2(ChangePileBaseAreaM2 { new_pile_base_area_m2: base.pile_base_area_m2.clone() })]
}
//#endregion 🔖️Inverse
