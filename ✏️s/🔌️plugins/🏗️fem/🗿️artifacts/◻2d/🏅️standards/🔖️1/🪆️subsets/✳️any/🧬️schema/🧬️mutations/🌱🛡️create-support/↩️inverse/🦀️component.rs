//! ↩️ Inverse for `CreateSupport` — always a `delete-support` of the created id.
use super::mutation::CreateSupport;
use crate::artifacts::fem2d::mutations::{delete_support, Fem2dMutation};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateSupport, _base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    vec![Fem2dMutation::DeleteSupport(delete_support::mutation::DeleteSupport { id: payload.support.id.clone() })]
}
//#endregion 🔖️Inverse
