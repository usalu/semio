//! 🔺️ Sparse diff builder for `CreateObject`.
use super::mutation::CreateObject;
use crate::artifacts::cad::diff::{CadDiff, CadObjectsDelta};
use crate::artifacts::cad::mutations::set_pane_objects_delta;
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateObject, _base: &CadSnapshot) -> CadDiff {
    let mut diff = CadDiff::default();
    set_pane_objects_delta(&mut diff, payload.pane, CadObjectsDelta { added: vec![payload.object.clone()], ..Default::default() });
    diff
}
//#endregion 🔖️Diff
