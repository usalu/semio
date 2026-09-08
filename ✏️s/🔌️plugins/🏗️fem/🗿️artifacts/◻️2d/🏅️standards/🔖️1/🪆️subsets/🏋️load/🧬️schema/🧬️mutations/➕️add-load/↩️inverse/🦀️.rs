//! ↩️ Inverse for `AddLoad` — a `remove-load` of the just-added load id, only if the case existed.
use super::AddLoad;
use crate::load_id;
use crate::mutations::{remove_load, Fem2dMutation};
use crate::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &AddLoad, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    if base.load_cases.iter().any(|case| case.id == payload.case_id) {
        vec![Fem2dMutation::RemoveLoad(remove_load::RemoveLoad { case_id: payload.case_id.clone(), load_id: load_id(&payload.load).to_string() })]
    } else {
        Vec::new()
    }
}
//#endregion 🔖️Inverse
