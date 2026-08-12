//! 🔺️ Sparse diff builder for `ChangeObjectTypology`.
use super::mutation::ChangeObjectTypology;
use crate::artifacts::cad::diff::{CadDiff, CadObjectPatchEntry, CadObjectsDelta};
use crate::artifacts::cad::mutations::{set_pane_objects_delta, CadObjectPatch};
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeObjectTypology, _base: &CadSnapshot) -> CadDiff {
    let mut diff = CadDiff::default();
    let patch = CadObjectPatch { typology: Some(payload.new_typology.clone()), ..Default::default() };
    set_pane_objects_delta(&mut diff, payload.pane, CadObjectsDelta { patched: vec![CadObjectPatchEntry { id: payload.object_id.clone(), patch }], ..Default::default() });
    diff
}
//#endregion 🔖️Diff
