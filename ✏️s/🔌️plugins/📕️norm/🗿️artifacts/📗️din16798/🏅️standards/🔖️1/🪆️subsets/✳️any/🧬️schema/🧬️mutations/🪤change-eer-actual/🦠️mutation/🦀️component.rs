//! 🔧 `change-eer-actual` payload — changes the Din16798 document's `eer_actual` (actual EER).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeEerActual
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeEerActual {
    pub new_eer_actual: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeEerActual {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "eer-actual", kind: "change-eer-actual", record: "ChangedEerActual" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        crate::artifacts::din16798::mutations::change_eer_actual::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_eer_actual::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change actual EER to {}", self.new_eer_actual)
    }
}
//#endregion 🔖️ChangeEerActual
