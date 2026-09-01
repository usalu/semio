//! 🔧 `change-occupancy` payload — changes the Din16798 document's `occupancy` (occupancy type).


use crate::artifacts::din16798::Din16798Snapshot;
use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::mutations::change_occupancy::ChangeOccupancy;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeOccupancy
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeOccupancy {
    pub new_occupancy: String,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeOccupancy {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "occupancy", kind: "change-occupancy", record: "ChangedOccupancy" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change occupancy type to \"{}\"", self.new_occupancy)
    }
}
//#endregion 🔖️ChangeOccupancy
