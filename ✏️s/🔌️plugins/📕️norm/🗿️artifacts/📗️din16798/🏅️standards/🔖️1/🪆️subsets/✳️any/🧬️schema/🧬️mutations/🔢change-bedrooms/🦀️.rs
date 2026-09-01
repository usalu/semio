//! 🔧 `change-bedrooms` payload — changes the Din16798 document's `bedrooms` (number of bedrooms).


use crate::artifacts::din16798::Din16798Snapshot;
use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::mutations::change_bedrooms::ChangeBedrooms;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeBedrooms
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeBedrooms {
    pub new_bedrooms: u32,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeBedrooms {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "bedrooms", kind: "change-bedrooms", record: "ChangedBedrooms" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change number of bedrooms to {}", self.new_bedrooms)
    }
}
//#endregion 🔖️ChangeBedrooms
