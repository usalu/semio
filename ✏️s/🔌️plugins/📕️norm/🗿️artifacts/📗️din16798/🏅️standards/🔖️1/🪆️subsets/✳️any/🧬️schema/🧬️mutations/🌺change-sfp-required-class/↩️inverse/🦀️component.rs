//! ↩️ `change-sfp-required-class` inverse — restores the pre-change `sfp_required_class` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_sfp_required_class::mutation::ChangeSfpRequiredClass;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeSfpRequiredClass, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeSfpRequiredClass(ChangeSfpRequiredClass { new_sfp_required_class: base.sfp_required_class.clone() })]
}
//#endregion 🔖️Inverse
