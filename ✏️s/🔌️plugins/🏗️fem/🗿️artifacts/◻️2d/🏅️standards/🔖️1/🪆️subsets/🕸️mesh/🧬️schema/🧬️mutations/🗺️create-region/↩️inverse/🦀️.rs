//! ↩️ Inverse for `CreateRegion` — always a `delete-region` of the created id.
use super::CreateRegion;
use crate::standards::v1::subsets::any::schema::mutations::delete_region;

use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<delete_region::DeleteRegion>>(payload: &CreateRegion, _base: &Fem2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Op::from(delete_region::DeleteRegion { id: payload.region.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
