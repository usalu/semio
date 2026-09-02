//! 🏗️ `change-mu` payload — changes the En1996 document's `mu` (friction coefficient mu).


use crate::artifacts::en1996::En1996Snapshot;
use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::mutations::change_mu::ChangeMu;

//#region 🔖️ChangeMu
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeMu {
    pub new_mu: f64,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeMu {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "mu", kind: "change-mu", record: "ChangedMu" };

    fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change friction coefficient mu to {}", self.new_mu)
    }
}
//#endregion 🔖️ChangeMu
