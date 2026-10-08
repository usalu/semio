//! ↩️ Inverse for `DeleteRegion` — recreates the captured region from `base`.
use super::DeleteRegion;
use crate::standards::v1::subsets::any::schema::mutations::{create_region,Fem2dMutation};

use crate::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteRegion, base: &Fem2dSnapshot) -> Result<Vec<Fem2dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    base.regions.iter().enumerate().find(|(_, item)| item.id == payload.id).map(|(at, item)| vec![Fem2dMutation::CreateRegion(create_region::CreateRegion { region: item.clone(), index: Some(at) })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
