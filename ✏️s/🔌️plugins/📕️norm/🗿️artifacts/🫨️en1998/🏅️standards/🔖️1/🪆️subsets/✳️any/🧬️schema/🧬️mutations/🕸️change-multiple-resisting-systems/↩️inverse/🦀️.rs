//! ↩️ `change-multiple-resisting-systems` inverse — restores the pre-change `multiple_resisting_systems` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_multiple_resisting_systems::ChangeMultipleResistingSystems;
use crate::mutations::En1998Mutation;
use crate::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeMultipleResistingSystems, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeMultipleResistingSystems(ChangeMultipleResistingSystems { new_multiple_resisting_systems: base.multiple_resisting_systems })]
}
//#endregion 🔖️Inverse
