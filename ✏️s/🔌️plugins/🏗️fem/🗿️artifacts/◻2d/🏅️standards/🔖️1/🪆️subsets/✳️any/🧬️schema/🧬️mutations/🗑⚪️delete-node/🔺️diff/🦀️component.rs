//! 🔺️ Sparse diff builder for `DeleteNode`.
use super::mutation::DeleteNode;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dNodesDelta};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteNode, _base: &Fem2dSnapshot) -> Fem2dDiff {
    Fem2dDiff { nodes: Some(Fem2dNodesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
