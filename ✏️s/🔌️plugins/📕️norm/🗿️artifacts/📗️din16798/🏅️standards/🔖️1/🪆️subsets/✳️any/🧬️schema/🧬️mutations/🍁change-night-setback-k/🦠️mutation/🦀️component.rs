//! 🔧 `change-night-setback-k` payload — changes the Din16798 document's `night_setback_k` (night setback temperature).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeNightSetbackK
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeNightSetbackK {
    pub new_night_setback_k: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeNightSetbackK {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "night-setback-k", kind: "change-night-setback-k", record: "ChangedNightSetbackK" };

    fn diff(&self, base: &Din16798Snapshot) -> Din16798Diff {
        crate::artifacts::din16798::mutations::change_night_setback_k::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_night_setback_k::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change night setback temperature to {}", self.new_night_setback_k)
    }
}
//#endregion 🔖️ChangeNightSetbackK
