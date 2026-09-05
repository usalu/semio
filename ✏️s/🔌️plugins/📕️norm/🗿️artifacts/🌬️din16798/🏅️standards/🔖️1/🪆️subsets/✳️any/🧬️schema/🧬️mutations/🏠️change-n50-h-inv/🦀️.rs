//! 🔧 `change-n50-h-inv` payload — changes the Din16798 document's `n50_h_inv` (n50 air change rate).


use crate::artifacts::din16798::Din16798Snapshot;
use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
//#region 🔖️ChangeN50HInv
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeN50HInv {
    pub new_n50_h_inv: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeN50HInv {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "n50-h-inv", kind: "change-n50-h-inv", record: "ChangedN50HInv" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change n50 air change rate to {}", self.new_n50_h_inv)
    }
}
//#endregion 🔖️ChangeN50HInv
