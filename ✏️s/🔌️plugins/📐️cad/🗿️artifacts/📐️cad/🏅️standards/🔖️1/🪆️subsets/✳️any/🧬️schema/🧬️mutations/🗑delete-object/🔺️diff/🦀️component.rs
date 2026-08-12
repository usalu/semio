//! 🔺️ Sparse diff builder for `DeleteObject`.
use super::mutation::DeleteObject;
use crate::artifacts::cad::diff::{CadDiff, CadObjectsDelta};
use crate::artifacts::cad::mutations::set_pane_objects_delta;
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteObject, _base: &CadSnapshot) -> CadDiff {
    let mut diff = CadDiff::default();
    set_pane_objects_delta(&mut diff, payload.pane, CadObjectsDelta { removed: vec![payload.object_id.clone()], ..Default::default() });
    diff
}
//#endregion 🔖️Diff
