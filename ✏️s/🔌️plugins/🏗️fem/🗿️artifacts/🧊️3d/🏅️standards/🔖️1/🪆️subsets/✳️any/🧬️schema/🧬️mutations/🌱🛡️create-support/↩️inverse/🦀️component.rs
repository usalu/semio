//! ↩️ Inverse for `CreateSupport` — always a `delete-support` of the created id.
use super::mutation::CreateSupport;
use crate::artifacts::fem3d::mutations::{delete_support, Fem3dMutation};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &CreateSupport, _base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
    vec![Fem3dMutation::DeleteSupport(delete_support::mutation::DeleteSupport { id: payload.support.id.clone() })]
}
//#endregion 🔖️Inverse
