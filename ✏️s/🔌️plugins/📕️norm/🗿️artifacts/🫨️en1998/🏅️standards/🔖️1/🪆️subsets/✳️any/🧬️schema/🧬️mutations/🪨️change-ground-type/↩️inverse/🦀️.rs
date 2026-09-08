//! ↩️ `change-ground-type` inverse — restores the pre-change `ground_type` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_ground_type::ChangeGroundType;
use crate::mutations::En1998Mutation;
use crate::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeGroundType, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeGroundType(ChangeGroundType { new_ground_type: base.ground_type.clone() })]
}
//#endregion 🔖️Inverse
