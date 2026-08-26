//! 🔧 `change-cooling-utilization-factor` payload — changes the Din16798 document's `cooling_utilization_factor` (cooling gain utilization factor).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeCoolingUtilizationFactor
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeCoolingUtilizationFactor {
    pub new_cooling_utilization_factor: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeCoolingUtilizationFactor {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "cooling-utilization-factor", kind: "change-cooling-utilization-factor", record: "ChangedCoolingUtilizationFactor" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        crate::artifacts::din16798::mutations::change_cooling_utilization_factor::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_cooling_utilization_factor::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change cooling gain utilization factor to {}", self.new_cooling_utilization_factor)
    }
}
//#endregion 🔖️ChangeCoolingUtilizationFactor
