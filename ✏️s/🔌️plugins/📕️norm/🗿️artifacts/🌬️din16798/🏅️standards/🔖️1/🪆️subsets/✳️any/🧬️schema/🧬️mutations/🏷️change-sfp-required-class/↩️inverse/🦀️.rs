//! ↩️ `change-sfp-required-class` inverse — restores the pre-change `sfp_required_class` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_sfp_required_class::ChangeSfpRequiredClass;
use crate::mutations::Din16798Mutation;
use crate::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeSfpRequiredClass, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeSfpRequiredClass(ChangeSfpRequiredClass { new_sfp_required_class: base.sfp_required_class })]
}
//#endregion 🔖️Inverse
