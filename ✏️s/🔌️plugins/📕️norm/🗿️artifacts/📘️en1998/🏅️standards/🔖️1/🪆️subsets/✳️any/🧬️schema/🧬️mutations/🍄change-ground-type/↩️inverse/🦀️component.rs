//! ↩️ `change-ground-type` inverse — restores the pre-change `ground_type` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_ground_type::mutation::ChangeGroundType;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeGroundType, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeGroundType(ChangeGroundType { new_ground_type: base.ground_type.clone() })]
}
//#endregion 🔖️Inverse
