//! 🔍 `change-fatigue-detail` — sets the En 1994 bridge fatigue detail category key scalar.


use crate::artifacts::en1994::{En1994Diff, En1994Mutation, En1994Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeFatigueDetail {
    pub new_fatigue_detail: String,
}

impl protocol::MutationKind<En1994Snapshot, En1994Mutation> for ChangeFatigueDetail {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fatigue-detail", kind: "change-fatigue-detail", record: "ChangedFatigueDetail" };

    fn diff(&self, base: &En1994Snapshot) -> protocol::MutationOutcome<<En1994Mutation as protocol::Mutation<En1994Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1994Snapshot) -> Vec<En1994Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change fatigue detail to \"{}\"", self.new_fatigue_detail)
    }
}
//#endregion 🔖️Payload
