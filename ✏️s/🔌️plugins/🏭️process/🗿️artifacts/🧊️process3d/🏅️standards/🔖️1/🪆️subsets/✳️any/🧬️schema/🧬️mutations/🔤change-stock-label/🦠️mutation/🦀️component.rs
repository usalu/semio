//! 🔧 `change-stock-label` payload — changes the document's single
//! [`Stock`](crate::artifacts::process3d::Stock) workpiece's `label`.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeStockLabel
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeStockLabel {
    pub new_label: String,
}

impl protocol::MutationKind<Process3dSnapshot, Process3dMutation> for ChangeStockLabel {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "stock", kind: "change-stock-label", record: "ChangedStockLabel" };

    fn diff(&self, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
        crate::artifacts::process3d::mutations::change_stock_label::diff::diff(self, base)
    }

    fn inverse(&self, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
        crate::artifacts::process3d::mutations::change_stock_label::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Rename stock to \"{}\"", self.new_label)
    }
}
//#endregion 🔖️ChangeStockLabel
