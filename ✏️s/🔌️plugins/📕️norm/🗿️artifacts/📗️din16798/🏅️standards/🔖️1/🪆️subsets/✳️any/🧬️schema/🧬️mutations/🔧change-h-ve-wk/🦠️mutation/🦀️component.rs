//! 🔧 `change-h-ve-wk` payload — changes the Din16798 document's `h_ve_w_k` (ventilation heat transfer coefficient).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeHVeWK
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeHVeWK {
    pub new_h_ve_w_k: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeHVeWK {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "h-ve-wk", kind: "change-h-ve-wk", record: "ChangedHVeWK" };

    fn diff(&self, base: &Din16798Snapshot) -> Din16798Diff {
        crate::artifacts::din16798::mutations::change_h_ve_w_k::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_h_ve_w_k::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change ventilation heat transfer coefficient to {}", self.new_h_ve_w_k)
    }
}
//#endregion 🔖️ChangeHVeWK
