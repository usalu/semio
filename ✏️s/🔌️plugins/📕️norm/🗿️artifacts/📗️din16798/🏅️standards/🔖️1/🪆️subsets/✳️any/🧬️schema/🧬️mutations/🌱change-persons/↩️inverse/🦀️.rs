//! ↩️ `change-persons` inverse — restores the pre-change `persons` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_persons::ChangePersons;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangePersons, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangePersons(ChangePersons { new_persons: base.persons.clone() })]
}
//#endregion 🔖️Inverse
