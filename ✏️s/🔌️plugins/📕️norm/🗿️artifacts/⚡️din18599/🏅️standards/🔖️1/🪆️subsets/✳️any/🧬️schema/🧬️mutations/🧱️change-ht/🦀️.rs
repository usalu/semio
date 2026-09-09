//! 🐫 `change-ht` payload — changes the Din18599 document's `h_t` (transmission heat transfer coefficient H_T [W/K]).

use crate::diff::Din18599Diff;
use crate::mutations::Din18599Mutation;
use crate::Din18599Snapshot;
//#region 🔖️ChangeHT
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeHT {
    pub new_h_t: f64,
}

impl protocol::MutationKind<Din18599Snapshot, Din18599Mutation> for ChangeHT {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "ht", kind: "change-ht", record: "ChangedHT" };

    fn diff(&self, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change transmission heat transfer coefficient H_T [W/K] to {}", self.new_h_t)
    }
}
//#endregion 🔖️ChangeHT
