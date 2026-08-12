//! 🔺️ Sparse diff builder for `CreateNode`.
use super::mutation::CreateNode;
use crate::artifacts::cad::diff::{CadDiff, CadNodesDelta};
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateNode, _base: &CadSnapshot) -> CadDiff {
    CadDiff { nodes: Some(CadNodesDelta { added: vec![payload.node.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
