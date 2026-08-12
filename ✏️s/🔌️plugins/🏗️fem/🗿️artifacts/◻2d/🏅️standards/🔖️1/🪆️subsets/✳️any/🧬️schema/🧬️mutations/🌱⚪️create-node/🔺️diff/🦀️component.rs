//! 🔺️ Sparse diff builder for `CreateNode`.
use super::mutation::CreateNode;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dNodesDelta};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateNode, _base: &Fem2dSnapshot) -> Fem2dDiff {
    Fem2dDiff { nodes: Some(Fem2dNodesDelta { added: vec![payload.node.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
