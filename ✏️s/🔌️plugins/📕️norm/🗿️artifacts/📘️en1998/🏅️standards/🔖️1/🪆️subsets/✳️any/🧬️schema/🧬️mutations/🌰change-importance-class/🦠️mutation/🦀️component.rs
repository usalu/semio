//! 🌰 `change-importance-class` payload — changes the En1998 document's `importance_class` (importance class).

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeImportanceClass
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeImportanceClass {
    pub new_importance_class: String,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeImportanceClass {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "importance-class", kind: "change-importance-class", record: "ChangedImportanceClass" };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        crate::artifacts::en1998::mutations::change_importance_class::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        crate::artifacts::en1998::mutations::change_importance_class::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change importance class to \"{}\"", self.new_importance_class)
    }
}
//#endregion 🔖️ChangeImportanceClass
