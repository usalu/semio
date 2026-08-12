//! 🔺️ Sparse diff builder for `CreateNode`.
use super::mutation::CreateNode;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dNodesDelta};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateNode, _base: &Fem3dSnapshot) -> Fem3dDiff {
    Fem3dDiff { nodes: Some(Fem3dNodesDelta { added: vec![payload.node.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
