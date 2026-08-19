//! 🔧 `change-duct-class` payload — changes the Din16798 document's `duct_class` (duct leakage class).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeDuctClass
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeDuctClass {
    pub new_duct_class: String,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeDuctClass {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "duct-class", kind: "change-duct-class", record: "ChangedDuctClass" };

    async fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        crate::artifacts::din16798::mutations::change_duct_class::diff::diff(self, base)
    }

    async fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_duct_class::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change duct leakage class to \"{}\"", self.new_duct_class)
    }
}
//#endregion 🔖️ChangeDuctClass
