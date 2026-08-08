//! ↩️ Inverse for `PatchObject`.
use crate::artifacts::cad::mutations::{reverse_object_patch, CadMutation, CadObjectPatch};
use crate::artifacts::cad::{cad_pane_objects, CadPaneId, CadProjection};

//#region 🔖️Inverse
pub fn inverse(base: &CadProjection, pane: CadPaneId, object_id: &str, patch: &CadObjectPatch) -> Vec<CadMutation> {
    cad_pane_objects(base, pane)
        .iter()
        .find(|object| object.id == *object_id)
        .map(|before| vec![CadMutation::PatchObject { pane, object_id: object_id.into(), patch: reverse_object_patch(before, patch) }])
        .unwrap_or_default()
}
//#endregion 🔖️Inverse
