//! 🔧 `change-rh-percent` payload — changes the Din16798 document's `rh_percent` (relative humidity).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeRhPercent
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeRhPercent {
    pub new_rh_percent: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeRhPercent {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "rh-percent", kind: "change-rh-percent", record: "ChangedRhPercent" };

    async fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        crate::artifacts::din16798::mutations::change_rh_percent::diff::diff(self, base)
    }

    async fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_rh_percent::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change relative humidity to {}", self.new_rh_percent)
    }
}
//#endregion 🔖️ChangeRhPercent
