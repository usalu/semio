//! ↩️ Inverse for `DeleteSupport` — recreates the captured support from `base`.
use super::DeleteSupport;
use crate::standards::v1::subsets::any::schema::mutations::create_support;

use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<create_support::CreateSupport>>(payload: &DeleteSupport, base: &Fem2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    base.supports.iter().enumerate().find(|(_, item)| item.id == payload.id).map(|(at, item)| vec![Op::from(create_support::CreateSupport { support: item.clone(), index: Some(at) })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
