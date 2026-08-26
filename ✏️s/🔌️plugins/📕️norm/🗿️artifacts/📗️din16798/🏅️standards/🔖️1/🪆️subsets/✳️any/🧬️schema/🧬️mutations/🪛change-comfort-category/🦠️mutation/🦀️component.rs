//! 🔧 `change-comfort-category` payload — changes the Din16798 document's `comfort_category` (comfort category).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeComfortCategory
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeComfortCategory {
    pub new_comfort_category: String,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeComfortCategory {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "comfort-category", kind: "change-comfort-category", record: "ChangedComfortCategory" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        crate::artifacts::din16798::mutations::change_comfort_category::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_comfort_category::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change comfort category to \"{}\"", self.new_comfort_category)
    }
}
//#endregion 🔖️ChangeComfortCategory
