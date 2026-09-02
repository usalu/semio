//! 🔧 `change-stock-label` payload — changes the document's single
//! [`Stock`](crate::artifacts::process3d::Stock) workpiece's `label`.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ChangeStockLabel
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeStockLabel {
    pub new_label: String,
}

impl protocol::MutationKind<Process3dSnapshot, Process3dMutation> for ChangeStockLabel {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "stock", kind: "change-stock-label", record: "ChangedStockLabel" };

    fn diff(&self, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Rename stock to \"{}\"", self.new_label)
    }
}
//#endregion 🔖️ChangeStockLabel
