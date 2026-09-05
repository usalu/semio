//! 🦏 `change-use-class` payload — changes the Din18599 document's `use_class` (building use class).


use crate::artifacts::din18599::Din18599Snapshot;
use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::Din18599Mutation;
//#region 🔖️ChangeUseClass
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeUseClass {
    pub new_use_class: crate::artifacts::din18599::UseClass,
}

impl protocol::MutationKind<Din18599Snapshot, Din18599Mutation> for ChangeUseClass {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "use-class", kind: "change-use-class", record: "ChangedUseClass" };

    fn diff(&self, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change building use class to {:?}", self.new_use_class)
    }
}
//#endregion 🔖️ChangeUseClass
