//! ↩️ Inverse for `CreateSupport` — always a `delete-support` of the created id.
use super::CreateSupport;
use crate::standards::v1::subsets::any::schema::mutations::{delete_support, Fem2dMutation};
use crate::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateSupport, _base: &Fem2dSnapshot) -> Result<Vec<Fem2dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Fem2dMutation::DeleteSupport(delete_support::DeleteSupport { id: payload.support.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
