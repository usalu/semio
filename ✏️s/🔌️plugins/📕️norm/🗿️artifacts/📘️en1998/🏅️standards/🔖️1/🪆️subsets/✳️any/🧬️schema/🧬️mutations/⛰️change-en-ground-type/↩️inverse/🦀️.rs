//! ↩️ `change-en-ground-type` inverse — restores the pre-change `en_ground_type` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_en_ground_type::ChangeEnGroundType;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeEnGroundType, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeEnGroundType(ChangeEnGroundType { new_en_ground_type: base.en_ground_type.clone() })]
}
//#endregion 🔖️Inverse
