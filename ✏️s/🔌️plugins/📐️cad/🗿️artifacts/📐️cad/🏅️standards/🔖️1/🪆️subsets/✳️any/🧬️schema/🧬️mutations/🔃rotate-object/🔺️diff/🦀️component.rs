//! 🔺️ Sparse diff builder for `RotateObject`.
use super::mutation::RotateObject;
use crate::artifacts::cad::diff::{CadDiff, CadObjectPatchEntry, CadObjectsDelta};
use crate::artifacts::cad::mutations::{set_pane_objects_delta, CadObjectPatch};
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &RotateObject, _base: &CadSnapshot) -> CadDiff {
    let mut diff = CadDiff::default();
    let patch = CadObjectPatch { orientation: Some(payload.new_orientation.clone()), ..Default::default() };
    set_pane_objects_delta(&mut diff, payload.pane, CadObjectsDelta { patched: vec![CadObjectPatchEntry { id: payload.object_id.clone(), patch }], ..Default::default() });
    diff
}
//#endregion 🔖️Diff
