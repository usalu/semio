//! ↩️ Inverse for `AddLoad` — a `remove-load` of the just-added load id, only if the case existed
//! and the id was genuinely new (a duplicate id is a warned no-op forward, so its inverse is empty).
use super::AddLoad;
use crate::load_id;
use crate::standards::v1::subsets::any::schema::mutations::{remove_load, Fem3dMutation};
use crate::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &AddLoad, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
    let new_load_id = load_id(&payload.load);
    match base.load_cases.iter().find(|case| case.id == payload.case_id) {
        Some(case) if case.loads.iter().all(|load| load_id(load) != new_load_id) => vec![Fem3dMutation::RemoveLoad(remove_load::RemoveLoad { case_id: payload.case_id.clone(), load_id: new_load_id.to_string() })],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
