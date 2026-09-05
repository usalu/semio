//! ↩️ `change-unit` inverse — restores the pre-change `unit` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1996::mutations::change_unit::ChangeUnit;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeUnit, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::ChangeUnit(ChangeUnit { new_unit: base.unit.clone() })]
}
//#endregion 🔖️Inverse
