//! 🔺️ Sparse diff builder for `ChangeObjectVisible`.
use super::mutation::ChangeObjectVisible;
use crate::artifacts::cad::diff::{CadDiff, CadObjectPatchEntry, CadObjectsDelta};
use crate::artifacts::cad::mutations::{set_pane_objects_delta, CadObjectPatch};
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeObjectVisible, _base: &CadSnapshot) -> CadDiff {
    let mut diff = CadDiff::default();
    let patch = CadObjectPatch { visible: Some(payload.new_visible.clone()), ..Default::default() };
    set_pane_objects_delta(&mut diff, payload.pane, CadObjectsDelta { patched: vec![CadObjectPatchEntry { id: payload.object_id.clone(), patch }], ..Default::default() });
    diff
}
//#endregion 🔖️Diff
