//! and the id was genuinely new (a duplicate id is a warned no-op forward, so its inverse is empty).
//! ↩️ Inverse for `AddLoad` — a `remove-load` of the just-added load id, only if the case existed
use super::AddLoad;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::load_id;
use crate::standards::v1::subsets::any::schema::mutations::remove_load;

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::standards::v1::subsets::any::schema::mutations::remove_load::RemoveLoad>>(payload: &AddLoad, base: &Fem3dSnapshotValue) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let new_load_id = load_id(&payload.load);
    match base.load_cases.iter().find(|case| case.id == payload.case_id) {
        Some(case) if case.loads.iter().all(|load| load_id(load) != new_load_id) => vec![Op::from(crate::standards::v1::subsets::any::schema::mutations::remove_load::RemoveLoad { case_id: payload.case_id.clone(), load_id: new_load_id.to_string() })],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
