//! ↩️ Inverse for `ReplaceLoad` — recovers the pre-mutation load from `base`.
use super::ReplaceLoad;
use crate::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use crate::{load_id, Fem2dSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &ReplaceLoad, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    base.load_cases
        .iter()
        .find(|case| case.id == payload.case_id)
        .and_then(|case| case.loads.iter().find(|load| load_id(load) == payload.load_id).cloned())
        .map(|load| vec![Fem2dMutation::ReplaceLoad(ReplaceLoad { case_id: payload.case_id.clone(), load_id: payload.load_id.clone(), new_load: Box::new(load) })])
        .unwrap_or_default()
}
//#endregion 🔖️Inverse
