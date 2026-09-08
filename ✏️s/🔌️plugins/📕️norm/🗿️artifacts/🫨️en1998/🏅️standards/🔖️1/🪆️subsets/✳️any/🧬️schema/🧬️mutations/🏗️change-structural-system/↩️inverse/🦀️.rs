//! ↩️ `change-structural-system` inverse — restores the pre-change `structural_system` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_structural_system::ChangeStructuralSystem;
use crate::mutations::En1998Mutation;
use crate::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeStructuralSystem, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeStructuralSystem(ChangeStructuralSystem { new_structural_system: base.structural_system.clone() })]
}
//#endregion 🔖️Inverse
