//! 🔧 `change-fan-t-run-h` payload — changes the Din16798 document's `fan_t_run_h` (fan running time).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeFanTRunH
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeFanTRunH {
    pub new_fan_t_run_h: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeFanTRunH {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fan-t-run-h", kind: "change-fan-t-run-h", record: "ChangedFanTRunH" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        crate::artifacts::din16798::mutations::change_fan_t_run_h::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_fan_t_run_h::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change fan running time to {}", self.new_fan_t_run_h)
    }
}
//#endregion 🔖️ChangeFanTRunH
