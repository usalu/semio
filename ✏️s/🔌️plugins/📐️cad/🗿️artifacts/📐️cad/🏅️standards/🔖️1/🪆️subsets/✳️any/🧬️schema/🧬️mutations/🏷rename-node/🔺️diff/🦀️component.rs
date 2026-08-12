//! 🔺️ Sparse diff builder for `RenameNode`.
use super::mutation::RenameNode;
use crate::artifacts::cad::diff::{CadDiff, CadNodePatchEntry, CadNodesDelta};
use crate::artifacts::cad::mutations::CadNodePatch;
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &RenameNode, _base: &CadSnapshot) -> CadDiff {
    CadDiff {
        nodes: Some(CadNodesDelta { patched: vec![CadNodePatchEntry { id: payload.node_id.clone(), patch: CadNodePatch { label: Some(payload.new_label.clone()) } }], ..Default::default() }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
