//! 🔧 `change-n-cycles-bridge` payload — changes the En1995 document's `n_cycles_bridge` (EN 1995 input).


use crate::artifacts::en1995::En1995Snapshot;
use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::En1995Mutation;
//#region 🔖️ChangeNCyclesBridge
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeNCyclesBridge {
    pub new_n_cycles_bridge: f64,
}

impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for ChangeNCyclesBridge {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "n-cycles-bridge", kind: "change-n-cycles-bridge", record: "ChangedNCyclesBridge" };

    fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1995Snapshot) -> Vec<En1995Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change n cycles bridge to {:?}", self.new_n_cycles_bridge)
    }
}
//#endregion 🔖️ChangeNCyclesBridge
