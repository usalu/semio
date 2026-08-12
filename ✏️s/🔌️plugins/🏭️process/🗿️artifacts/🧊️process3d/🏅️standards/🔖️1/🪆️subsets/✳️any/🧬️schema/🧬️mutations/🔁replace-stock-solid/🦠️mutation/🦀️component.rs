//! 🔁 `replace-stock-solid` payload — whole-value swap of the document's single
//! [`Stock`](crate::artifacts::process3d::Stock) workpiece's `solid` (large structured field, per
//! `📓️derivation-rules.md` rule 1).

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::{Process3dSnapshot, SolidSpec};
use serde::{Deserialize, Serialize};

//#region 🔖️ReplaceStockSolid
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceStockSolid {
    pub new_solid: SolidSpec,
}

impl protocol::MutationKind<Process3dSnapshot, Process3dMutation> for ReplaceStockSolid {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "stock", kind: "replace-stock-solid", record: "ReplacedStockSolid" };

    fn diff(&self, base: &Process3dSnapshot) -> Process3dDiff {
        crate::artifacts::process3d::mutations::replace_stock_solid::diff::diff(self, base)
    }

    fn inverse(&self, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
        crate::artifacts::process3d::mutations::replace_stock_solid::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        "Replace stock solid".to_string()
    }
}
//#endregion 🔖️ReplaceStockSolid
