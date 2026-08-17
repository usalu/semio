//! 🔧 `change-persons` payload — changes the Din16798 document's `persons` (number of persons).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangePersons
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePersons {
    pub new_persons: u32,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangePersons {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "persons", kind: "change-persons", record: "ChangedPersons" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        crate::artifacts::din16798::mutations::change_persons::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_persons::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change number of persons to {}", self.new_persons)
    }
}
//#endregion 🔖️ChangePersons
