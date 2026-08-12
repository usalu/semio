//! 🔧 `change-h-tr-wk` payload — changes the Din16798 document's `h_tr_w_k` (transmission heat transfer coefficient).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeHTrWK
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeHTrWK {
    pub new_h_tr_w_k: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeHTrWK {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "h-tr-wk", kind: "change-h-tr-wk", record: "ChangedHTrWK" };

    fn diff(&self, base: &Din16798Snapshot) -> Din16798Diff {
        crate::artifacts::din16798::mutations::change_h_tr_w_k::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_h_tr_w_k::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change transmission heat transfer coefficient to {}", self.new_h_tr_w_k)
    }
}
//#endregion 🔖️ChangeHTrWK
