//! 🔺️ Sparse diff builder for `RenameObject`.
use super::mutation::RenameObject;
use crate::artifacts::cad::diff::{CadDiff, CadObjectPatchEntry, CadObjectsDelta};
use crate::artifacts::cad::mutations::{set_pane_objects_delta, CadObjectPatch};
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &RenameObject, _base: &CadSnapshot) -> CadDiff {
    let mut diff = CadDiff::default();
    let patch = CadObjectPatch { label: Some(payload.new_label.clone()), ..Default::default() };
    set_pane_objects_delta(&mut diff, payload.pane, CadObjectsDelta { patched: vec![CadObjectPatchEntry { id: payload.object_id.clone(), patch }], ..Default::default() });
    diff
}
//#endregion 🔖️Diff
