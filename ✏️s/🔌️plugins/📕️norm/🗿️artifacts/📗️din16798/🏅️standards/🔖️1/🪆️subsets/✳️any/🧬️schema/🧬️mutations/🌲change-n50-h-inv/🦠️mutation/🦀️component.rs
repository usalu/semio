//! 🔧 `change-n50-h-inv` payload — changes the Din16798 document's `n50_h_inv` (n50 air change rate).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeN50HInv
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeN50HInv {
    pub new_n50_h_inv: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeN50HInv {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "n50-h-inv", kind: "change-n50-h-inv", record: "ChangedN50HInv" };

    fn diff(&self, base: &Din16798Snapshot) -> Din16798Diff {
        crate::artifacts::din16798::mutations::change_n50_h_inv::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_n50_h_inv::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change n50 air change rate to {}", self.new_n50_h_inv)
    }
}
//#endregion 🔖️ChangeN50HInv
