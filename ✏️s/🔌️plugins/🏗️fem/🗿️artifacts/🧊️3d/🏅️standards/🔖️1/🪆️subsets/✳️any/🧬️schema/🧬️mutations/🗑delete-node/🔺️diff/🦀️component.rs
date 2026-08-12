//! 🔺️ Sparse diff builder for `DeleteNode`.
use super::mutation::DeleteNode;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dNodesDelta};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteNode, _base: &Fem3dSnapshot) -> Fem3dDiff {
    Fem3dDiff { nodes: Some(Fem3dNodesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
