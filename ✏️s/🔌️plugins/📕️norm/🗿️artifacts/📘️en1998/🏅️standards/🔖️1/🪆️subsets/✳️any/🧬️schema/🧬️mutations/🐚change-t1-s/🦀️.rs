//! 🐚 `change-t1-s` payload — changes the En1998 document's `t1_s` (fundamental period T1 [s]).


use crate::artifacts::en1998::En1998Snapshot;
use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::mutations::change_t1_s::ChangeT1S;

//#region 🔖️ChangeT1S
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeT1S {
    pub new_t1_s: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeT1S {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "t1-s", kind: "change-t1-s", record: "ChangedT1S" };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change fundamental period T1 [s] to {}", self.new_t1_s)
    }
}
//#endregion 🔖️ChangeT1S
