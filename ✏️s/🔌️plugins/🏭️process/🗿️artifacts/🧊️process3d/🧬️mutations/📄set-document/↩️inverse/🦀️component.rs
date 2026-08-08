//! ↩️ Inverse for `SetDocument`.
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dDocument;

//#region 🔖️Inverse
pub fn inverse(base: &Process3dDocument, _document: &Process3dDocument) -> Vec<Process3dMutation> {
    vec![Process3dMutation::SetDocument { document: base.clone() }]
}
//#endregion 🔖️Inverse
