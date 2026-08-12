//! 🔧 `change-hr-delta-tc` payload — changes the Din16798 document's `hr_delta_t_c` (heat recovery temperature difference).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeHrDeltaTC
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeHrDeltaTC {
    pub new_hr_delta_t_c: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeHrDeltaTC {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "hr-delta-tc", kind: "change-hr-delta-tc", record: "ChangedHrDeltaTC" };

    fn diff(&self, base: &Din16798Snapshot) -> Din16798Diff {
        crate::artifacts::din16798::mutations::change_hr_delta_t_c::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_hr_delta_t_c::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change heat recovery temperature difference to {}", self.new_hr_delta_t_c)
    }
}
//#endregion 🔖️ChangeHrDeltaTC
