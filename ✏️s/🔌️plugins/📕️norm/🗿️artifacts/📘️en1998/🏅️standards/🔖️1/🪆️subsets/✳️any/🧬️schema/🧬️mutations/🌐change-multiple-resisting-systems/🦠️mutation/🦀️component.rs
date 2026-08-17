//! 🌐 `change-multiple-resisting-systems` payload — changes the En1998 document's `multiple_resisting_systems` (multiple resisting systems flag).

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeMultipleResistingSystems
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeMultipleResistingSystems {
    pub new_multiple_resisting_systems: bool,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeMultipleResistingSystems {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "multiple-resisting-systems", kind: "change-multiple-resisting-systems", record: "ChangedMultipleResistingSystems" };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        crate::artifacts::en1998::mutations::change_multiple_resisting_systems::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        crate::artifacts::en1998::mutations::change_multiple_resisting_systems::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change multiple resisting systems flag to {}", self.new_multiple_resisting_systems)
    }
}
//#endregion 🔖️ChangeMultipleResistingSystems
