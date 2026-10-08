//! ↩️ Inverse for `DeleteSupport` — recreates the captured support from `base`.
use super::DeleteSupport;
use crate::standards::v1::subsets::any::schema::mutations::{create_support,Fem3dMutation};

use crate::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteSupport, base: &Fem3dSnapshot) -> Result<Vec<Fem3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    base.supports.iter().enumerate().find(|(_, item)| item.id == payload.id).map(|(at, item)| vec![Fem3dMutation::CreateSupport(create_support::CreateSupport { support: item.clone(), index: Some(at) })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
