//! ↩️ `change-service-class` inverse — restores the pre-change `service_class` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_service_class::ChangeServiceClass;
use crate::mutations::En1995Mutation;
use crate::En1995Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeServiceClass, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    vec![En1995Mutation::ChangeServiceClass(ChangeServiceClass { new_service_class: base.service_class.clone() })]
}
//#endregion 🔖️Inverse
