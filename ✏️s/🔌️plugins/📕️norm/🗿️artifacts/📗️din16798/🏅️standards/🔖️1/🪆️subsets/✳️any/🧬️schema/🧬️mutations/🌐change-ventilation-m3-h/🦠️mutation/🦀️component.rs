//! 🔧 `change-ventilation-m3-h` payload — changes the Din16798 document's `ventilation_m3_h` (ventilation air flow).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeVentilationM3H
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeVentilationM3H {
    pub new_ventilation_m3_h: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeVentilationM3H {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "ventilation-m3-h", kind: "change-ventilation-m3-h", record: "ChangedVentilationM3H" };

    async fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        crate::artifacts::din16798::mutations::change_ventilation_m3_h::diff::diff(self, base)
    }

    async fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_ventilation_m3_h::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change ventilation air flow to {}", self.new_ventilation_m3_h)
    }
}
//#endregion 🔖️ChangeVentilationM3H
