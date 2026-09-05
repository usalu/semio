//! 🔧 `change-h-tr-wk` payload — changes the Din16798 document's `h_tr_w_k` (transmission heat transfer coefficient).


use crate::artifacts::din16798::Din16798Snapshot;
use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
//#region 🔖️ChangeHTrWK
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeHTrWK {
    pub new_h_tr_w_k: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeHTrWK {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "h-tr-wk", kind: "change-h-tr-wk", record: "ChangedHTrWK" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change transmission heat transfer coefficient to {}", self.new_h_tr_w_k)
    }
}
//#endregion 🔖️ChangeHTrWK
