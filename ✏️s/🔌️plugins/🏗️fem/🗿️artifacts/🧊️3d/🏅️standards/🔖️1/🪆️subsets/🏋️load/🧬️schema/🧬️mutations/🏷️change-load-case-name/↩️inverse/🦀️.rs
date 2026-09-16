//! ↩️ Inverse for `ChangeLoadCaseName` — recovers the pre-mutation name from `base`.
use super::ChangeLoadCaseName;
use crate::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use crate::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ChangeLoadCaseName, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
    base.load_cases.iter().find(|case| case.id == payload.case_id).map(|case| vec![Fem3dMutation::ChangeLoadCaseName(ChangeLoadCaseName { case_id: payload.case_id.clone(), new_name: case.name.clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
