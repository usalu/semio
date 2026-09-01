//! 🔧 `change-use-fem` payload — changes the En1992 document's `use_fem` (EN 1992 input).


use crate::artifacts::en1992::En1992Snapshot;
use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::mutations::change_use_fem::ChangeUseFem;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeUseFem
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeUseFem {
    pub new_use_fem: bool,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for ChangeUseFem {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "use-fem", kind: "change-use-fem", record: "ChangedUseFem" };

    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1992Snapshot) -> Vec<En1992Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change use fem to {:?}", self.new_use_fem)
    }
}
//#endregion 🔖️ChangeUseFem
