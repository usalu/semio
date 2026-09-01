//! 🧱 `change-deck-type` — sets the En 1994 composite deck profile type scalar.


use crate::artifacts::en1994::{En1994Diff, En1994Mutation, En1994Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeDeckType {
    pub new_deck_type: String,
}

impl protocol::MutationKind<En1994Snapshot, En1994Mutation> for ChangeDeckType {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "deck-type", kind: "change-deck-type", record: "ChangedDeckType" };

    fn diff(&self, base: &En1994Snapshot) -> protocol::MutationOutcome<<En1994Mutation as protocol::Mutation<En1994Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1994Snapshot) -> Vec<En1994Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change deck type to \"{}\"", self.new_deck_type)
    }
}
//#endregion 🔖️Payload
