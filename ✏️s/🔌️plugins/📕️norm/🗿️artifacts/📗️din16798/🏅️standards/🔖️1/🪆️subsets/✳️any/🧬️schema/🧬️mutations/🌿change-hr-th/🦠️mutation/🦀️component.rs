//! 🔧 `change-hr-th` payload — changes the Din16798 document's `hr_t_h` (heat recovery operating time).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeHrTH
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeHrTH {
    pub new_hr_t_h: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeHrTH {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "hr-th", kind: "change-hr-th", record: "ChangedHrTH" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        crate::artifacts::din16798::mutations::change_hr_t_h::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_hr_t_h::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change heat recovery operating time to {}", self.new_hr_t_h)
    }
}
//#endregion 🔖️ChangeHrTH
