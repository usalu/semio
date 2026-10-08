//! ↩️ Inverse for `RemoveLoad` — recreates the captured load (via `add-load`) from `base`.
use super::RemoveLoad;
use crate::load_id;
use crate::standards::v1::subsets::any::schema::mutations::{add_load,Fem2dMutation};

use crate::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &RemoveLoad, base: &Fem2dSnapshot) -> Result<Vec<Fem2dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    base.load_cases
        .iter()
        .find(|case| case.id == payload.case_id)
        .and_then(|case| case.loads.iter().position(|load| load_id(load) == payload.load_id).map(|at| (at, case.loads[at].clone())))
        .map(|(at, load)| vec![Fem2dMutation::AddLoad(add_load::AddLoad { case_id: payload.case_id.clone(), load: Box::new(load), index: Some(at) })])
        .unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
