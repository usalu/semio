//! 🔺️ Sparse diff builder for `DeleteNode`.
use super::mutation::DeleteNode;
use crate::artifacts::cad::diff::{CadDiff, CadNodesDelta};
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteNode, _base: &CadSnapshot) -> CadDiff {
    CadDiff { nodes: Some(CadNodesDelta { removed: vec![payload.node_id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
